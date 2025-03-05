#[cfg(test)]
pub mod utils {
    use cosmwasm_std::{Uint128, Decimal};

    /// Calculate required collateral based on debt amount and collateralization ratio
    pub fn calculate_required_collateral(debt: Uint128, ratio_percentage: u64) -> Uint128 {
        // Convert ratio to decimal form (e.g., 110% -> 1.1)
        let ratio = Decimal::percent(ratio_percentage);
        let base = Decimal::percent(100);
        
        // Calculate required collateral: debt * (ratio/100)
        // Note: To avoid division with Uint128, we use the following steps:
        // 1. Multiply debt by ratio (debt * ratio)
        // 2. Divide the result by 100 (equivalent to dividing by base)
        let intermediate = debt * ratio;
        let required = intermediate * Uint128::from(1u64) / Uint128::from(base.atomics().u128() / Decimal::one().atomics().u128());
        
        required
    }
    
    /// Check if a position is sufficiently collateralized
    pub fn is_sufficiently_collateralized(collateral: Uint128, debt: Uint128, ratio_percentage: u64) -> bool {
        let required = calculate_required_collateral(debt, ratio_percentage);
        collateral >= required
    }
    
    /// Calculate liquidation price
    pub fn calculate_liquidation_price(debt: Uint128, collateral: Uint128, ratio_percentage: u64) -> Decimal {
        let ratio = Decimal::percent(ratio_percentage);
        let base = Decimal::percent(100);
        
        // Liquidation price = (debt * ratio) / (collateral * base)
        // We cannot directly divide due to type constraints, so we convert to integer ratios
        let debt_value = Decimal::from_ratio(debt, Uint128::from(1u64));
        let collateral_value = Decimal::from_ratio(collateral, Uint128::from(1u64));
        
        debt_value * ratio / (collateral_value * base)
    }
}
