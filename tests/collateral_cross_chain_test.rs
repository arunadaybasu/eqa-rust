#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        from_json, Addr, Uint128, to_json_binary, Binary, coins,
    };
    use collateral_manager::{
        ExecuteMsg, QueryMsg, CollateralResponse, InstantiateMsg,
    };
    
    #[test]
    fn test_cw20_token_handling() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        // Initialize contract
        let info = mock_info("admin", &[]);
        let msg = InstantiateMsg {
            admin: None,
            register_cross_chain: Some(false),
        };
        
        let res = collateral_manager::instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(2, res.attributes.len());
        
        // Test receiving tokens
        let token_addr = "usdc_axelar_token";
        let amount = Uint128::new(1_000_000); // 1 USDC
        
        let msg = ExecuteMsg::ReceiveTokens { 
            token_addr: token_addr.to_string(), 
            amount 
        };
        
        let info = mock_info("user1", &[]);
        let res = collateral_manager::execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        
        // Check the collateral was properly updated
        let query_msg = QueryMsg::GetCollateralInfo {};
        let res: CollateralResponse = from_json(collateral_manager::query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
        
        assert_eq!(res.usdc_axelar, Uint128::new(1_000_000));
        assert_eq!(res.total_locked, Uint128::new(1_000_000));
        
        // Test cross-chain sending
        let amount = Uint128::new(500_000); // 0.5 USDC
        let msg = ExecuteMsg::SendViaAxelar { 
            destination_chain: "ethereum".to_string(),
            destination_address: "0x123...".to_string(),
            amount,
        };
        
        // Need to send fee with the transaction
        let info = mock_info("user1", &coins(1_000_000, "uluna"));
        let res = collateral_manager::execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        
        // Check that we have the right number of messages and attributes
        assert!(res.messages.len() > 0);
        assert!(res.attributes.iter().any(|attr| attr.key == "action" && attr.value == "cross_chain_send_axelar"));
    }
    
    #[test]
    fn test_handle_cross_chain_messages() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        // Initialize contract
        let info = mock_info("admin", &[]);
        let msg = InstantiateMsg {
            admin: None,
            register_cross_chain: Some(true),
        };
        
        let _res = collateral_manager::instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        // Create a mock Axelar message
        let payload = to_json_binary(&serde_json::json!({
            "action": "deposit",
            "recipient": "user1",
            "amount": "1000000"
        })).unwrap();
        
        let msg = ExecuteMsg::HandleAxelarMessage { 
            source_chain: "ethereum".to_string(),
            source_address: "0x456...".to_string(),
            payload,
        };
        
        // Message should come from Axelar gateway
        let info = mock_info("axelar_gateway_address", &[]);
        let res = collateral_manager::execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        
        // Check that collateral was updated
        let query_msg = QueryMsg::GetCollateralInfo {};
        let res: CollateralResponse = from_json(collateral_manager::query(deps.as_ref(), env, query_msg).unwrap()).unwrap();
        
        assert_eq!(res.usdc_axelar, Uint128::new(1_000_000));
        assert_eq!(res.total_locked, Uint128::new(1_000_000));
    }
}
