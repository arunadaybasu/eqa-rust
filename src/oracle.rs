// Shared Oracle Interface for the EQA system
use cosmwasm_std::Decimal;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Common messages that can be used by contracts that need price data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OracleQueryMsg {
    GetPrice { denom: String },
    GetExchangeRate { base_denom: String, quote_denom: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct PriceResponse {
    pub denom: String,
    pub price: Decimal,
    pub last_updated: u64, // timestamp
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ExchangeRateResponse {
    pub base_denom: String,
    pub quote_denom: String,
    pub rate: Decimal,
    pub last_updated: u64, // timestamp
}

// Helper function to calculate deviation from peg
pub fn calculate_deviation_from_peg(market_price: Decimal) -> Decimal {
    let peg = Decimal::one();
    if market_price > peg {
        market_price - peg
    } else {
        peg - market_price
    }
}

// Helper function to calculate dynamic fee based on price deviation
pub fn calculate_dynamic_fee(market_price: Decimal) -> Decimal {
    let deviation = calculate_deviation_from_peg(market_price);
    
    if deviation > Decimal::percent(1) {
        Decimal::percent(5) // 5% fee when EQA deviates more than 1%
    } else {
        Decimal::percent(1) // Default 1% fee
    }
}
