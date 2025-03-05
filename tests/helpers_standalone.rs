#[cfg(test)]
pub mod helpers {
    use cosmwasm_std::{Uint128, Decimal};

    // Calculate fee amount
    pub fn calculate_fee(amount: Uint128, fee_percentage: u64) -> Uint128 {
        amount * Decimal::percent(fee_percentage)
    }

    // Calculate required collateral
    pub fn calculate_required_collateral(debt: Uint128, ratio_percentage: u64) -> Uint128 {
        // This avoids the division issue by computing directly
        let ratio_decimal = Decimal::percent(ratio_percentage);
        debt * ratio_decimal / Uint128::new(100)
    }

    // Check if sufficiently collateralized
    pub fn is_sufficiently_collateralized(collateral: Uint128, debt: Uint128, ratio_percentage: u64) -> bool {
        let required = calculate_required_collateral(debt, ratio_percentage);
        collateral >= required
    }
}
