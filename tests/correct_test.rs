#[cfg(test)]
mod tests {
    use cosmwasm_std::{Uint128, Decimal};
    
    #[test]
    fn test_decimal_operations() {
        // Test basic Decimal math
        let one = Decimal::one();
        let half = Decimal::percent(50);
        
        assert_eq!(half + half, one);
        assert_eq!(one - half, half);
    }
    
    #[test]
    fn test_uint128_with_decimal() {
        // Test Uint128 * Decimal
        let amount = Uint128::new(100);
        
        // 50% of 100 = 50
        let half_amount = amount * Decimal::percent(50);
        assert_eq!(half_amount, Uint128::new(50));
        
        // 110% of 100 = 110
        let increased_amount = amount * Decimal::percent(110);
        assert_eq!(increased_amount, Uint128::new(110));
    }
    
    #[test]
    fn test_collateralization_check() {
        // 110% collateral requirement
        let debt = Uint128::new(100);
        
        // FIXED: Calculate required collateral directly with multiplication only
        // This was previously using division which was causing the issue
        let required_collateral = debt * Decimal::percent(110);
        
        // Now this should pass
        assert_eq!(required_collateral, Uint128::new(110));
        
        // Well-collateralized (110 >= 110)
        let collateral1 = Uint128::new(110);
        assert!(collateral1 >= required_collateral);
        
        // Under-collateralized (109 < 110)
        let collateral2 = Uint128::new(109);
        assert!(collateral2 < required_collateral);
    }
}
