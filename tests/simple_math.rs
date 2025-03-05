#[cfg(test)]
mod tests {
    use cosmwasm_std::{Uint128, Decimal};

    #[test]
    fn test_addition() {
        let a = Uint128::new(100);
        let b = Uint128::new(200);
        assert_eq!(a + b, Uint128::new(300));
    }

    #[test]
    fn test_decimal_multiplication() {
        let amount = Uint128::new(100);
        let multiplier = Decimal::percent(110); // 1.1
        let result = amount * multiplier;
        assert_eq!(result, Uint128::new(110));
    }

    #[test]
    fn test_decimal_percentage() {
        // 110% as a decimal is 1.1
        let percent_110 = Decimal::percent(110);
        let one_point_one = Decimal::one() + Decimal::percent(10);
        assert_eq!(percent_110, one_point_one);
    }
}
