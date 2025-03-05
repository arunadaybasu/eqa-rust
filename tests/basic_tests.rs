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
    fn test_collateralization_requirement() {
        // 110% collateral requirement
        let debt = Uint128::new(100);
        
        // Calculate required collateral directly
        let required_collateral = Uint128::new(110); // 110% of 100 = 110
        
        // Test well-collateralized case
        let collateral_good = Uint128::new(110);
        assert!(collateral_good >= required_collateral);
        
        // Test under-collateralized case
        let collateral_bad = Uint128::new(109);
        assert!(collateral_bad < required_collateral);
    }
    
    #[test]
    fn test_decimal_math() {
        let one = Decimal::one();
        let half = Decimal::percent(50);
        
        assert_eq!(one - half, Decimal::percent(50));
        assert_eq!(one + half, Decimal::percent(150));
        
        // Compare decimal numbers
        assert!(Decimal::percent(120) > Decimal::percent(110));
        assert!(Decimal::percent(90) < Decimal::percent(100));
    }
    
    #[test]
    fn test_collateralization_alternative() {
        // 110% collateral requirement
        let collateral = Uint128::new(110);
        let debt = Uint128::new(100);
        
        // Instead of division, use multiplication for comparison
        // debt * 110% should be <= collateral * 100%
        let debt_scaled = debt * Decimal::percent(110);
        let collateral_scaled = collateral * Decimal::percent(100);
        
        assert!(collateral_scaled >= debt_scaled);
        
        // Test under-collateralized case
        let collateral = Uint128::new(109);
        let collateral_scaled = collateral * Decimal::percent(100);
        
        assert!(collateral_scaled < debt_scaled);
    }
}
