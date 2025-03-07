#[cfg(test)]
mod tests {
    use cosmwasm_std::{Decimal, Uint128};
    
    #[test]
    fn test_basic_calculations() {
        // Test basic math operations
        let a = Uint128::new(100);
        let b = Uint128::new(50);
        
        assert_eq!(a + b, Uint128::new(150));
        assert_eq!(a - b, Uint128::new(50));
        assert_eq!(a * Uint128::from(2u8), Uint128::new(200));
        assert_eq!(a / Uint128::from(2u8), Uint128::new(50));
    }
    
    #[test]
    fn test_decimal_math() {
        // Test decimal operations
        let a = Decimal::percent(50); // 0.5
        let b = Decimal::percent(20); // 0.2
        
        assert_eq!(a + b, Decimal::percent(70));
        assert_eq!(a - b, Decimal::percent(30));
        
        // Test multiplication with Uint128
        let value = Uint128::new(100);
        assert_eq!(value * a, Uint128::new(50));
    }
    
    #[test]
    fn test_collateralization_requirement() {
        // Setup test values
        let debt = Uint128::new(100_000);           // 100,000 EQA
        let collateral = Uint128::new(150_000);     // 150,000 USDC
        let ratio = Decimal::percent(150);         // 150%
        
        // Calculate required collateral: debt * ratio
        let required = debt * ratio;
        
        // Convert to a comparable unit (remove decimal places)
        let required_uint = required / Uint128::from(100u64);
        
        // Check if properly collateralized
        assert!(collateral >= required_uint);
    }
    
    #[test]
    fn test_collateralization_alternative() {
        // Setup test values
        let debt = Uint128::new(100_000);
        let collateral = Uint128::new(150_000);
        let ratio = 150u128; // 150%
        
        // Calculate required collateral: (debt * ratio) / 100
        let required = debt * Uint128::from(ratio) / Uint128::from(100u128);
        
        // Check if properly collateralized
        assert!(collateral >= required);
        assert_eq!(required, Uint128::new(150_000));
    }
}
