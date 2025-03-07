#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::from_json;
    use registry::{
        InstantiateMsg, ExecuteMsg, QueryMsg, 
        ContractAddressResponse, AllContractsResponse
    };
    
    #[test]
    fn test_registry_basic_functions() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        // Initialize registry
        let info = mock_info("admin", &[]);
        let msg = InstantiateMsg {
            admin: None, // Default to sender
        };
        
        let res = registry::instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(2, res.attributes.len());
        
        // Set contract address
        let msg = ExecuteMsg::SetContractAddress { 
            name: "test_contract".to_string(), 
            address: "terra1test".to_string() 
        };
        
        let res = registry::execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert!(res.attributes.iter().any(|attr| attr.key == "name" && attr.value == "test_contract"));
        
        // Set another contract address
        let msg = ExecuteMsg::SetContractAddress { 
            name: "test_contract2".to_string(), 
            address: "terra1test2".to_string() 
        };
        
        let res = registry::execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert!(res.attributes.iter().any(|attr| attr.key == "name" && attr.value == "test_contract2"));
        
        // Query contract address
        let query_msg = QueryMsg::GetContractAddress { name: "test_contract".to_string() };
        let res: ContractAddressResponse = from_json(registry::query(deps.as_ref(), env.clone(), query_msg).unwrap()).unwrap();
        
        assert_eq!(res.name, "test_contract");
        assert_eq!(res.address, "terra1test");
        
        // Query all contracts
        let query_msg = QueryMsg::GetAllContracts {};
        let res: AllContractsResponse = from_json(registry::query(deps.as_ref(), env, query_msg).unwrap()).unwrap();
        
        assert_eq!(res.contracts.len(), 2);
    }

    #[test]
    fn test_registry_admin_functions() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        
        // Initialize registry
        let info = mock_info("admin", &[]);
        let msg = InstantiateMsg {
            admin: None, // Default to sender
        };
        
        let _res = registry::instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        
        // Update admin
        let msg = ExecuteMsg::UpdateConfig { 
            new_admin: Some(cosmwasm_std::Addr::unchecked("new_admin")) 
        };
        
        let res = registry::execute(deps.as_mut(), env.clone(), info, msg).unwrap();
        assert!(res.attributes.iter().any(|attr| attr.key == "action" && attr.value == "update_config"));
        
        // Try to set address with new admin
        let new_admin_info = mock_info("new_admin", &[]);
        let msg = ExecuteMsg::SetContractAddress { 
            name: "test_contract3".to_string(), 
            address: "terra1test3".to_string() 
        };
        
        let res = registry::execute(deps.as_mut(), env.clone(), new_admin_info, msg).unwrap();
        assert!(res.attributes.iter().any(|attr| attr.key == "name" && attr.value == "test_contract3"));
        
        // Try with unauthorized user
        let unauthorized_info = mock_info("unauthorized", &[]);
        let msg = ExecuteMsg::SetContractAddress { 
            name: "test_contract4".to_string(), 
            address: "terra1test4".to_string() 
        };
        
        let res = registry::execute(deps.as_mut(), env, unauthorized_info, msg);
        assert!(res.is_err()); // Should fail
    }
}
