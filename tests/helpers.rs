#[cfg(test)]
pub mod test_helpers {
    use cosmwasm_std::Uint128;
    
    // A simple helper to calculate fees - like in the actual contract
    pub fn calculate_fee(amount: Uint128, percentage: u64) -> Uint128 {
        let fee_percent = percentage as u128;
        amount * Uint128::from(fee_percent) / Uint128::from(100u128)
    }

    // A mock version of execute_mint for testing
    pub fn mock_mint(amount: Uint128) -> Uint128 {
        let fee_amount = calculate_fee(amount, 1); // 1% fee
        amount - fee_amount
    }

    // A mock version of execute_redeem for testing
    pub fn mock_redeem(balance: Uint128, amount: Uint128) -> Result<Uint128, &'static str> {
        if balance < amount {
            return Err("Insufficient funds");
        }
        let fee_amount = calculate_fee(amount, 1); // 1% fee
        Ok(amount - fee_amount)
    }
}
