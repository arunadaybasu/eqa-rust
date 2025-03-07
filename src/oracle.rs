use cosmwasm_std::{Decimal, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

// Common messages that can be used by contracts that need price data
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OracleQueryMsg {
    GetPrice { denom: String },
    GetExchangeRate { base_denom: String, quote_denom: String },
    GetPrices {},
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OracleResponse {
    pub price: Decimal,
    pub last_updated: u64,
    pub source: String,
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

/// Calculate dynamic fee based on price deviation from target
/// 
/// There are two modes:
/// 1. Single parameter: Uses default target of 1.0 (the peg)
/// 2. Two parameters: Uses the provided target price
pub fn calculate_dynamic_fee(current_price: Decimal, target_price: Option<Decimal>) -> StdResult<Decimal> {
    // Use provided target price or default to 1.0 (the peg)
    let target = target_price.unwrap_or_else(Decimal::one);
    
    // Calculate deviation from target price (absolute value)
    let deviation = if current_price > target {
        current_price - target
    } else {
        target - current_price
    };
    
    // Base fee is 0.1%
    let base_fee = Decimal::permille(1);
    
    // Additional fee is proportional to deviation (1% fee per 1% deviation)
    let additional_fee = deviation;
    
    // Calculate total fee (base + additional)
    let total_fee = base_fee + additional_fee;
    
    // Cap at 5% maximum fee
    let max_fee = Decimal::percent(5);
    if total_fee > max_fee {
        return Ok(max_fee);
    }
    
    Ok(total_fee)
}

/// Calculate the token price based on collateral and supply
pub fn calculate_price(collateral: Uint128, supply: Uint128) -> StdResult<Decimal> {
    if supply.is_zero() {
        return Ok(Decimal::zero());
    }
    
    Ok(Decimal::from_ratio(collateral, supply))
}

/// Calculate ideal trade size based on deviation and market depth
pub fn calculate_optimal_trade_size(
    deviation: Decimal,
    market_depth: Uint128,
) -> StdResult<Uint128> {
    // A simple model: optimal size = market_depth * deviation * 10
    // This is a simplified version; real models would be more complex
    let percentage = deviation.to_string().parse::<f64>().unwrap_or(0.0);
    let trade_size = market_depth.u128() as f64 * percentage * 10.0;
    
    Ok(Uint128::new(trade_size as u128))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dynamic_fee_calculation() {
        // No deviation = base fee only (0.1%)
        let fee = calculate_dynamic_fee(Decimal::one(), Some(Decimal::one())).unwrap();
        assert_eq!(fee, Decimal::permille(1));
        
        // 1% deviation = 1.1% fee
        let fee = calculate_dynamic_fee(Decimal::percent(99), Some(Decimal::one())).unwrap();
        assert_eq!(fee, Decimal::permille(11));
        
        // 5% deviation = 5.1% fee (capped at 5%)
        let fee = calculate_dynamic_fee(Decimal::percent(95), Some(Decimal::one())).unwrap();
        assert_eq!(fee, Decimal::percent(5));
        
        // Test single parameter version (implicit target of 1.0)
        let fee = calculate_dynamic_fee(Decimal::percent(95), None).unwrap();
        assert_eq!(fee, Decimal::percent(5));
    }

    #[test]
    fn test_price_calculation() {
        // 100 tokens with 100 collateral = 1.0 price
        let price = calculate_price(Uint128::new(100), Uint128::new(100)).unwrap();
        assert_eq!(price, Decimal::one());
        
        // 200 tokens with 100 collateral = 0.5 price
        let price = calculate_price(Uint128::new(100), Uint128::new(200)).unwrap();
        assert_eq!(price, Decimal::percent(50));
        
        // 50 tokens with 100 collateral = 2.0 price
        let price = calculate_price(Uint128::new(100), Uint128::new(50)).unwrap();
        assert_eq!(price, Decimal::percent(200));
        
        // 0 tokens = 0 price to avoid division by zero
        let price = calculate_price(Uint128::new(100), Uint128::zero()).unwrap();
        assert_eq!(price, Decimal::zero());
    }
}
