use cosmwasm_std::{Decimal, Uint128, StdResult, StdError};

/// Calculates the token price based on collateral and supply
pub fn calculate_price(collateral: Uint128, supply: Uint128) -> StdResult<Decimal> {
    if supply.is_zero() {
        return Err(StdError::generic_err("Supply cannot be zero"));
    }
    
    // Convert to decimals for division
    let coll_decimal = Decimal::from_ratio(collateral, Uint128::new(1));
    let supply_decimal = Decimal::from_ratio(supply, Uint128::new(1));
    
    // Calculate price = collateral / supply
    let price = coll_decimal / supply_decimal;
    
    Ok(price)
}

/// Calculates fee based on price deviation from peg
pub fn calculate_dynamic_fee(current_price: Decimal, target_price: Decimal) -> StdResult<Decimal> {
    // Calculate deviation from target price
    let deviation = if current_price > target_price {
        current_price - target_price
    } else {
        target_price - current_price
    };
    
    // Base fee is 0.1%
    let base_fee = Decimal::permille(1);
    
    // Additional fee based on deviation (1% fee per 1% deviation)
    let additional_fee = deviation;
    
    // Calculate total fee (base + additional)
    let total_fee = base_fee + additional_fee;
    
    // Cap at 5%
    let max_fee = Decimal::percent(5);
    if total_fee > max_fee {
        return Ok(max_fee);
    }
    
    Ok(total_fee)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_price_calculation() {
        // 100 collateral / 100 supply = 1.0 price
        let price = calculate_price(Uint128::new(100), Uint128::new(100)).unwrap();
        assert_eq!(price, Decimal::one());
        
        // 110 collateral / 100 supply = 1.1 price
        let price = calculate_price(Uint128::new(110), Uint128::new(100)).unwrap();
        assert_eq!(price, Decimal::percent(110));
        
        // 90 collateral / 100 supply = 0.9 price
        let price = calculate_price(Uint128::new(90), Uint128::new(100)).unwrap();
        assert_eq!(price, Decimal::percent(90));
    }
    
    #[test]
    fn test_dynamic_fee() {
        // No deviation = base fee (0.1%)
        let fee = calculate_dynamic_fee(Decimal::one(), Decimal::one()).unwrap();
        assert_eq!(fee, Decimal::permille(1));
        
        // 1% deviation = 1.1% fee
        let fee = calculate_dynamic_fee(Decimal::percent(99), Decimal::one()).unwrap();
        assert_eq!(fee, Decimal::permille(11));
        
        // 10% deviation = 10.1% fee, capped at 5%
        let fee = calculate_dynamic_fee(Decimal::percent(90), Decimal::one()).unwrap();
        assert_eq!(fee, Decimal::percent(5)); // Capped at 5%
    }
}
