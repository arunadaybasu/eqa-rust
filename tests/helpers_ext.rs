// This file provides helper functions that are directly available
// without requiring complex imports
#[cfg(test)]
pub mod test_helpers {
    use cosmwasm_std::{Uint128, Decimal};
    
    pub fn calculate_fee(amount: Uint128, percentage: u64) -> Uint128 {
        amount * Decimal::percent(percentage)
    }
    
    pub fn mock_mint(amount: Uint128) -> Uint128 {
        let fee_amount = calculate_fee(amount, 1);
        amount - fee_amount
    }
    
    pub fn mock_redeem(balance: Uint128, amount: Uint128) -> Result<Uint128, &'static str> {
        if balance < amount {
            return Err("Insufficient funds");
        }
        let fee_amount = calculate_fee(amount, 1);
        Ok(amount - fee_amount)
    }
    
    pub fn calc_collateral_requirement(debt: Uint128, ratio_percentage: u64) -> Uint128 {
        let ratio = Decimal::percent(ratio_percentage);
        (debt * ratio).into()
    }
    
    pub fn is_sufficiently_collateralized(collateral: Uint128, debt: Uint128, ratio_percentage: u64) -> bool {
        let required = calc_collateral_requirement(debt, ratio_percentage);
        collateral >= required
    }
}
