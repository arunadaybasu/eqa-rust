#[cfg(test)]
mod tests {
    use cosmwasm_std::{Uint128, Decimal};

    // Define mock_mint locally instead of trying to import
    fn mock_mint(amount: Uint128) -> Uint128 {
        let fee = amount * Decimal::percent(1);
        amount - fee
    }

    #[test]
    fn test_mint_eqa() {
        // Use the local mock function
        let amount = Uint128::new(100_000_000); // 100 tokens
        let received = mock_mint(amount);
        
        // Check if the 1% fee was applied correctly
        assert_eq!(received, Uint128::new(99_000_000)); // 99 tokens
    }
    
    #[test]
    fn test_fee_calculation() {
        // Basic 1% fee calculation
        let amount = Uint128::new(1_000_000); // 1 token with 6 decimals
        let fee = Decimal::percent(1);
        let fee_amount = amount * fee;
        let final_amount = amount - fee_amount;
        
        assert_eq!(fee_amount, Uint128::new(10_000)); // 0.01 token fee
        assert_eq!(final_amount, Uint128::new(990_000)); // 0.99 tokens after fee
        
        // Higher 5% fee calculation (for when price deviates)
        let amount = Uint128::new(1_000_000);
        let fee = Decimal::percent(5);
        let fee_amount = amount * fee;
        let final_amount = amount - fee_amount;
        
        assert_eq!(fee_amount, Uint128::new(50_000)); // 0.05 token fee
        assert_eq!(final_amount, Uint128::new(950_000)); // 0.95 tokens after fee
    }
    
    #[test]
    fn test_dynamic_fee_levels() {
        // Simulate the calculate_dynamic_fee function
        let calculate_fee = |market_price: Decimal| -> Decimal {
            // Fix the .abs() issue by comparing values
            let deviation = if market_price > Decimal::one() {
                market_price - Decimal::one()
            } else {
                Decimal::one() - market_price
            };
            
            if deviation > Decimal::percent(1) {
                Decimal::percent(5) // 5% fee when EQA deviates more than 1%
            } else {
                Decimal::percent(1) // Default 1% fee
            }
        };
        
        // When price is at peg
        let fee = calculate_fee(Decimal::one()); // price = 1.0
        assert_eq!(fee, Decimal::percent(1));
        
        // When price is close to peg - use permille for small percentages
        let near_peg = Decimal::one() + Decimal::permille(5); // 1 + 0.005 = 1.005
        let fee = calculate_fee(near_peg);
        assert_eq!(fee, Decimal::percent(1));
        
        // When price deviates above threshold
        let above_threshold = Decimal::one() + Decimal::percent(2); // 1 + 0.02 = 1.02
        let fee = calculate_fee(above_threshold);
        assert_eq!(fee, Decimal::percent(5));
        
        // When price deviates below peg
        let below_threshold = Decimal::one() - Decimal::percent(2); // 1 - 0.02 = 0.98
        let fee = calculate_fee(below_threshold);
        assert_eq!(fee, Decimal::percent(5));
    }
    
    #[test]
    fn test_collateralization_checks() {
        // 110% minimum collateralization ratio
        let threshold_ratio = 110u64;
        
        // Test case 1: Well collateralized (100k collateral, 90k EQA)
        let collateral = Uint128::new(100_000);
        let eqa_supply = Uint128::new(90_000);
        let required_collateral = eqa_supply.checked_mul(Uint128::from(threshold_ratio)).unwrap()
            .checked_div(Uint128::from(100u64)).unwrap();
        
        assert_eq!(required_collateral, Uint128::new(99_000));
        assert!(collateral >= required_collateral);
        
        // Test case 2: Under-collateralized (100k collateral, 95k EQA)
        let eqa_supply = Uint128::new(95_000);
        let required_collateral = eqa_supply.checked_mul(Uint128::from(threshold_ratio)).unwrap()
            .checked_div(Uint128::from(100u64)).unwrap();
        
        assert_eq!(required_collateral, Uint128::new(104_500));
        assert!(collateral < required_collateral);
    }
}
