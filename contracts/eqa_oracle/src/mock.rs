// This file provides mocks to simulate Terra Classic Oracle for testing
use cosmwasm_std::{Deps, Decimal};

// Mocked TerraQuerier for testing without the terra-cosmwasm dependency
pub struct MockTerraQuerier {}

pub struct ExchangeRateResponse {
    pub exchange_rate: MockDecimal,
}

pub struct MockDecimal {
    numerator: u128,
    denominator: u128,
}

impl MockDecimal {
    pub fn numerator(&self) -> u128 {
        self.numerator
    }
    
    pub fn denominator(&self) -> u128 {
        self.denominator
    }
}

impl MockTerraQuerier {
    pub fn new(_querier: &Deps) -> Self {
        Self {}
    }
    
    pub fn query_exchange_rate(&self, _base_denom: String, _quote_denom: String) -> Result<ExchangeRateResponse, cosmwasm_std::StdError> {
        // In tests, always return a 1:1 rate for simplicity
        Ok(ExchangeRateResponse {
            exchange_rate: MockDecimal {
                numerator: 1,
                denominator: 1,
            },
        })
    }
}
