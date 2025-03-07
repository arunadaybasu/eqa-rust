#[cfg(test)]
mod tests {
    use cosmwasm_std::{Decimal, QueryRequest, WasmQuery};
    use equilibria_smart_contracts::mocks::mock_dependencies_with_custom_querier;
    use equilibria_smart_contracts::oracle::{OracleQueryMsg, OracleResponse};

    #[test]
    fn test_mock_custom_querier() {
        // Create dependencies with custom querier
        let custom_price = Decimal::percent(105); // 1.05
        let deps = mock_dependencies_with_custom_querier(Some(custom_price));
        
        // Create a query to the oracle
        let query_msg = OracleQueryMsg::GetPrice { denom: "eqa".to_string() };
        
        // Convert query to raw wasm query
        let wasm_query = WasmQuery::Smart {
            contract_addr: "terra1oracle_mainnet".to_string(),
            msg: cosmwasm_std::to_json_binary(&query_msg).unwrap(),
        };
        
        let query = QueryRequest::Wasm(wasm_query);
        
        // Execute the query
        let binary = deps.querier.query(&query).unwrap();
        
        // Parse the response
        let oracle_response: OracleResponse = cosmwasm_std::from_json(&binary).unwrap();
        
        // Verify the mocked price was returned
        assert_eq!(oracle_response.price, custom_price);
        assert_eq!(oracle_response.source, "mock_oracle");
    }
    
    #[test]
    fn test_mock_custom_querier_with_default_price() {
        // Create dependencies with custom querier and default price (1.0)
        let deps = mock_dependencies_with_custom_querier(None);
        
        // Create a query to the oracle
        let query_msg = OracleQueryMsg::GetPrice { denom: "eqa".to_string() };
        
        // Convert query to raw wasm query
        let wasm_query = WasmQuery::Smart {
            contract_addr: "terra1oracle_mainnet".to_string(),
            msg: cosmwasm_std::to_json_binary(&query_msg).unwrap(),
        };
        
        let query = QueryRequest::Wasm(wasm_query);
        
        // Execute the query
        let binary = deps.querier.query(&query).unwrap();
        
        // Parse the response
        let oracle_response: OracleResponse = cosmwasm_std::from_json(&binary).unwrap();
        
        // Verify the default price (1.0) was returned
        assert_eq!(oracle_response.price, Decimal::one());
    }
    
    #[test]
    fn test_mocks_integration() {
        // This test creates mock dependencies and verifies the price oracle works with defaults
        let deps = mock_dependencies_with_custom_querier(None);
        
        // Test that default price is 1.0
        let query = QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: "terra1oracle_mainnet".to_string(),
            msg: cosmwasm_std::to_json_binary(&OracleQueryMsg::GetPrice { 
                denom: "eqa".to_string() 
            }).unwrap(),
        });
        
        let res: OracleResponse = deps.querier.query(&query).unwrap();
        assert_eq!(res.price, Decimal::one());
        
        // Test with a different price
        let deps = mock_dependencies_with_custom_querier(Some(Decimal::percent(95))); // 0.95
        
        let query = QueryRequest::Wasm(WasmQuery::Smart {
            contract_addr: "terra1oracle_mainnet".to_string(),
            msg: cosmwasm_std::to_json_binary(&OracleQueryMsg::GetPrice { 
                denom: "eqa".to_string() 
            }).unwrap(),
        });
        
        let res: OracleResponse = deps.querier.query(&query).unwrap();
        assert_eq!(res.price, Decimal::percent(95));
    }
}
