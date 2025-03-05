#[cfg(test)]
mod tests {
    use cosmwasm_std::{Uint128, Decimal};

    #[test]
    fn test_decimal_multiplication() {
        // Basic test - 100 * 1.1 = 110
        let amount = Uint128::new(100);
        let multiplier = Decimal::percent(110); // 1.1
        let result = amount * multiplier;
        assert_eq!(result, Uint128::new(110));
    }

    #[test]
    fn test_collateralization() {
        // 100 debt with 110% collateral requirement needs 110 collateral
        let debt = Uint128::new(100);
        let required_ratio = Decimal::percent(110); // 1.1
        
        // Calculate required collateral - directly multiply
        let required_collateral = debt * required_ratio;
        
        // Should be 110
        assert_eq!(required_collateral, Uint128::new(110));
        
        // Test sufficient collateral (110 >= 110)
        let good_collateral = Uint128::new(110);
        assert!(good_collateral >= required_collateral);
        
        // Test insufficient collateral (109 < 110)
        let bad_collateral = Uint128::new(109);
        assert!(bad_collateral < required_collateral);
    }
}
