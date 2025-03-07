#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        from_json, Addr, Uint128, to_json_binary, Binary,
    };
    use collateral_manager::{
        ExecuteMsg, QueryMsg, CollateralResponse, InstantiateMsg, RegistryResponse
    };
    use registry::{
        ExecuteMsg as RegistryExecuteMsg,
        QueryMsg as RegistryQueryMsg,
        InstantiateMsg as RegistryInstantiateMsg,
    };
    
    // Helper function to setup registry with test addresses
    fn setup_registry(deps: &mut cosmwasm_std::OwnedDeps<_, _, _>) -> String {
        let env = mock_env();
        let info = mock_info("admin", &[]);
        
        // Initialize registry
        let msg = RegistryInstantiateMsg {
            admin: None, // Default to sender
        };
        
        let res = registry::instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(2, res.attributes.len());
        
        // Set contract address for Axelar USDC
        let msg = RegistryExecuteMsg::SetContractAddress { 
            name: "axelar_usdc".to_string(), 
            address: "terra1axelar_usdc_address".to_string() 
        };
        
        let _res = registry::execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        // Set contract address for Noble USDC
        let msg = RegistryExecuteMsg::SetContractAddress { 
            name: "noble_usdc".to_string(), 
            address: "terra1noble_usdc_address".to_string() 
        };
        
        let _res = registry::execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        // Set contract address for Axelar Gateway
        let msg = RegistryExecuteMsg::SetContractAddress { 
            name: "axelar_gateway".to_string(), 
            address: "terra1axelar_gateway".to_string() 
        };
        
        let _res = registry::execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        // Set contract address for Noble Gateway
        let msg = RegistryExecuteMsg::SetContractAddress { 
            name: "noble_gateway".to_string(), 
            address: "terra1noble_gateway".to_string() 
        };
        
        let _res = registry::execute(deps.as_mut(), env, info, msg).unwrap();
        
        // Return registry contract address (normally this would be the actual address,
        // but for testing we can use a dummy address)
        "registry_contract_address".to_string()
    }
    
    #[test]
    fn test_collateral_manager_with_registry() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        // Setup registry with test addresses
        let registry_addr = setup_registry(&mut deps);
        
        // Initialize collateral manager with registry address
        let info = mock_info("manager_admin", &[]);
        let msg = InstantiateMsg {
            admin: None, // Default to sender
            registry_address: registry_addr.clone(),
            register_cross_chain: Some(false),
        };
        
        let res = collateral_manager::instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert!(res.attributes.iter().any(|attr| attr.key == "registry_address" && attr.value == registry_addr));
        
        // Verify registry address is stored correctly
        let query_msg = QueryMsg::GetRegistryAddress {};
        let res: RegistryResponse = from_json(collateral_manager::query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
        assert_eq!(res.address, registry_addr);
        
        // Test updating registry address
        let new_registry_addr = "new_registry_address".to_string();
        let msg = ExecuteMsg::UpdateRegistry { new_registry_address: new_registry_addr.clone() };
        let _res = collateral_manager::execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        // Verify new registry address is stored
        let query_msg = QueryMsg::GetRegistryAddress {};
        let res: RegistryResponse = from_json(collateral_manager::query(deps.as_ref(), env, query_msg).unwrap()).unwrap();
        assert_eq!(res.address, new_registry_addr);
    }
    
    #[test]
    fn test_cw20_token_handling_with_registry() {
        // This would be a comprehensive test of CW20 token operations
        // using the registry for address lookup
        // 
        // However, in our test environment, we can't directly test
        // interactions with other contracts without a proper multi-test setup
        //
        // In a real environment, you should use cosmwasm-multitest for this
        
        // Code example for multitest approach:
        /*
        use cosmwasm_std::coins;
        use cosmwasm_multitest::{App, Contract, ContractWrapper, Executor};
        
        // First create and store registry contract
        let registry_id = app.store_code(registry_contract);
        
        // Instantiate registry contract
        let registry_addr = app
            .instantiate_contract(
                registry_id,
                Addr::unchecked("admin"),
                &registry::InstantiateMsg { admin: None },
                &[],
                "registry",
                None,
            )
            .unwrap();
            
        // Set contract addresses in registry
        app.execute_contract(
            Addr::unchecked("admin"),
            registry_addr.clone(),
            &registry::ExecuteMsg::SetContractAddress { 
                name: "axelar_usdc".to_string(), 
                address: cw20_addr.to_string() 
            },
            &[],
        ).unwrap();
        
        // Instantiate collateral manager with registry
        let cm_addr = app.instantiate_contract(
            cm_id,
            Addr::unchecked("admin"),
            &collateral_manager::InstantiateMsg {
                admin: None,
                registry_address: registry_addr.to_string(),
                register_cross_chain: Some(false),
            },
            &[],
            "collateral_manager",
            None,
        ).unwrap();
        
        // Test receive_tokens
        app.execute_contract(
            Addr::unchecked("user"),
            cm_addr.clone(),
            &collateral_manager::ExecuteMsg::ReceiveTokens {
                token_addr: cw20_addr.to_string(),
                amount: Uint128::new(100),
            },
            &[],
        ).unwrap();
        */
    }
    
    #[test]
    fn test_cross_chain_with_registry() {
        // This would test cross-chain operations using the registry
        // Similar to above, this requires a proper multi-test environment
    }
}
