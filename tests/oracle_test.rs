#[cfg(test)]
mod tests {
    use cosmwasm_std::{from_json, to_json_binary, Decimal, Uint128};
    
    // Import modules from the library
    use equilibria_smart_contracts::oracle::{OracleQueryMsg, calculate_dynamic_fee};
    
    #[test]
    fn test_oracle_price_calculation() {
        // This test simulates an oracle query and validates fee calculation
        
        // Setup mock query
        let query_msg = OracleQueryMsg::GetPrice { denom: "eqa".to_string() };
        let _query_bin = to_json_binary(&query_msg).unwrap();
        
        // Simulate a price from the oracle (1.02 - slightly above peg)
        let oracle_price = Decimal::percent(102);
        
        // Simulate a response
        let response_json = format!(r#"{{"price":{}, "last_updated":123456789, "source":"test"}}"#, oracle_price);
        let response_bin = response_json.into_bytes();
        
        // Parse response
        let response: equilibria_smart_contracts::oracle::OracleResponse = 
            from_json(&response_bin).unwrap();
        
        // Validate response
        assert_eq!(response.price, oracle_price);
        assert_eq!(response.last_updated, 123456789);
        assert_eq!(response.source, "test");
        
        // Calculate fee based on deviation
        let target_price = Decimal::one(); // 1.0 is the peg
        let fee = calculate_dynamic_fee(response.price, Some(target_price)).unwrap();
        
        // Deviation is 2%, so fee should be base (0.1%) + 2% = 2.1%, capped at 5%
        assert_eq!(fee, Decimal::permille(21));
        
        // Calculate redemption amount with fee
        let amount = Uint128::new(100_000_000); // 100 tokens
        let fee_amount = amount * fee;
        let redemption_amount = amount - fee_amount;
        
        // Fee should be 2.1% of 100 = 2.1
        assert_eq!(fee_amount, Uint128::new(2_100_000));
        // Redemption amount should be 100 - 2.1 = 97.9
        assert_eq!(redemption_amount, Uint128::new(97_900_000));
    }
    
    #[test]
    fn test_price_impact() {
        // Setup a mock market with 10,000 tokens and $15,000 collateral
        // This gives a price of $1.50 per token
        let supply = Uint128::new(10_000);
        let collateral = Uint128::new(15_000);
        
        // Calculate the price
        let price = equilibria_smart_contracts::oracle::calculate_price(collateral, supply).unwrap();
        assert_eq!(price, Decimal::percent(150)); // $1.50
        
        // Simulate adding 1,000 more tokens (10% increase in supply)
        let new_supply = supply + Uint128::new(1_000);
        let new_price = equilibria_smart_contracts::oracle::calculate_price(collateral, new_supply).unwrap();
        
        // New price should be $15,000 / 11,000 = $1.36
        let expected_price = Decimal::from_ratio(collateral, new_supply);
        assert_eq!(new_price, expected_price);
        
        // Calculate price impact (change from $1.50 to $1.36)
        let price_impact = (price - new_price) / price * Decimal::percent(100);
        
        // Price impact should be about 9.1%
        assert!(price_impact > Decimal::percent(9) && price_impact < Decimal::percent(10));
    }
}
