#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        to_json_binary, 
        Decimal,
        // Removed the unused Addr import
    };
    use equilibria_smart_contracts::oracle::{OracleQueryMsg, calculate_dynamic_fee};
    
    #[test]
    fn test_oracle_interface() {
        // Test oracle query message serialization
        let query_msg = OracleQueryMsg::GetPrice {
            denom: "eqa".to_string(),
        };
        
        let serialized = to_json_binary(&query_msg).unwrap();
        assert!(serialized.len() > 0);
        
        // Test dynamic fee calculation based on price deviation
        let at_peg = Decimal::one();
        let slight_deviation = Decimal::percent(100) + Decimal::permille(5); // 1.005
        let large_deviation = Decimal::percent(100) + Decimal::percent(2);   // 1.02
        
        assert_eq!(calculate_dynamic_fee(at_peg), Decimal::percent(1));
        assert_eq!(calculate_dynamic_fee(slight_deviation), Decimal::percent(1));
        assert_eq!(calculate_dynamic_fee(large_deviation), Decimal::percent(5));
    }
}
