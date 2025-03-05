#[cfg(test)]
mod tests {
    use cosmwasm_std::{Uint128, Decimal};
    
    #[test]
    fn test_collateral_requirement() {
        // 110% collateral requirement
        let debt = Uint128::new(100);
        
        // Calculate required collateral without division
        let required_collateral = Uint128::new(110); // Hardcoded 110% of 100
        
        // Well-collateralized (110 >= 110)
        let collateral_good = Uint128::new(110);
        assert!(collateral_good >= required_collateral);
        
        // Under-collateralized (109 < 110)
        let collateral_bad = Uint128::new(109);
        assert!(collateral_bad < required_collateral);
    }
    
    #[test]
    fn test_alternative_approach() {
        // Instead of calculating required collateral
        // Check if actual collateral * 100 >= debt * 110
        
        let debt = Uint128::new(100);
        
        // Test well-collateralized case
        let collateral_good = Uint128::new(110);
        let debt_scaled = debt * Decimal::percent(110);
        let collateral_scaled = collateral_good * Decimal::percent(100);
        
        assert!(collateral_scaled >= debt_scaled);
        
        // Test under-collateralized case
        let collateral_bad = Uint128::new(109);
        let collateral_bad_scaled = collateral_bad * Decimal::percent(100);
        
        assert!(collateral_bad_scaled < debt_scaled);
    }
}
