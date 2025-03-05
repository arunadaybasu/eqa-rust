#[cfg(test)]
mod tests {
    use cosmwasm_std::{Uint128, Decimal};

    #[test]
    fn test_basic_calculations() {
        // Test 1% fee on 100 tokens
        let amount = Uint128::new(100);
        let fee_percent = Decimal::percent(1);
        let fee_amount = amount * fee_percent;
        let final_amount = amount - fee_amount;
        
        assert_eq!(fee_amount, Uint128::new(1));
        assert_eq!(final_amount, Uint128::new(99));
    }
    
    #[test]
    fn test_collateralization_direct() {
        // Define values
        let debt = Uint128::new(100);
        let required_ratio = Decimal::percent(110); // 110%
        
        // Calculate required collateral (100 * 110% = 110)
        let required_collateral = debt * required_ratio;
        assert_eq!(required_collateral, Uint128::new(110));
        
        // Well-collateralized case
        let collateral_good = Uint128::new(110);
        assert!(collateral_good >= required_collateral);
        
        // Under-collateralized case  
        let collateral_bad = Uint128::new(109);
        assert!(collateral_bad < required_collateral);
    }
}
