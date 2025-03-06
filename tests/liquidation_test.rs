#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        testing::mock_dependencies,
        Uint128,
    };
    use equilibria_smart_contracts::state::{CollateralState, COLLATERAL};
    
    #[test]
    fn test_basic_liquidation_math() {
        // This is a basic test of liquidation math 
        // that doesn't depend on the full contract
        
        // Set up
        let collateral = Uint128::new(100_000);
        let debt = Uint128::new(90_000);
        let threshold_ratio = 110u128; // 110%
        
        // Calculate required collateral: (debt * threshold_ratio) / 100
        // Using integer math to avoid Decimal division issues
        let required_collateral = debt.checked_mul(Uint128::from(threshold_ratio))
            .unwrap()
            .checked_div(Uint128::from(100u128))
            .unwrap();
        
        // Should be solvent (collateral >= required_collateral)
        assert_eq!(required_collateral, Uint128::new(99_000));
        assert!(collateral >= required_collateral);
        
        // Calculate collateralization ratio: (collateral * 100) / debt
        let collateral_ratio = collateral
            .checked_mul(Uint128::from(100u128))
            .unwrap()
            .checked_div(debt)
            .unwrap();
        
        assert_eq!(collateral_ratio, Uint128::new(111));
        assert!(collateral_ratio.u128() >= threshold_ratio);
        
        // Test insolvency
        let high_debt = Uint128::new(95_000);
        let required_collateral_high = high_debt
            .checked_mul(Uint128::from(threshold_ratio))
            .unwrap()
            .checked_div(Uint128::from(100u128))
            .unwrap();
        
        // Should be insolvent (collateral < required_collateral_high)
        assert_eq!(required_collateral_high, Uint128::new(104_500));
        assert!(collateral < required_collateral_high);
    }
    
    #[test]
    fn test_collateral_storage() {
        let mut deps = mock_dependencies();
        
        // Setup collateral
        let collateral_state = CollateralState {
            usdc_axelar: Uint128::new(60_000),
            usdc_noble: Uint128::new(40_000),
            total_locked: Uint128::new(100_000),
        };
        
        // Save to storage
        COLLATERAL.save(deps.as_mut().storage, &collateral_state).unwrap();
        
        // Retrieve and verify
        let stored = COLLATERAL.load(deps.as_ref().storage).unwrap();
        assert_eq!(stored.usdc_axelar, Uint128::new(60_000));
        assert_eq!(stored.usdc_noble, Uint128::new(40_000));
        assert_eq!(stored.total_locked, Uint128::new(100_000));
    }
}
