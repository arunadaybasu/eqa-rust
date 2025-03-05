#[cfg(test)]
mod tests {
    use cosmwasm_std::{Uint128, Decimal};

    #[test]
    fn test_basic_math() {
        // Simple fee calculation test
        let amount = Uint128::new(100);
        let fee_percent = Decimal::percent(1);
        let fee_amount = amount * fee_percent;
        
        assert_eq!(fee_amount, Uint128::new(1));
        assert_eq!(amount - fee_amount, Uint128::new(99));
    }
    
    #[test]
    fn test_decimal_operations() {
        // Test decimal operations that work
        let one = Decimal::one();
        let half = Decimal::percent(50);
        
        assert_eq!(one - half, half);
        assert_eq!(one + half, Decimal::percent(150));
        
        // Testing permille for fractional percentages
        let small = Decimal::permille(5); // 0.5%
        assert_eq!(small, Decimal::percent(50) / Decimal::from_ratio(100u128, 1u128));
    }
}
