#[cfg(test)]
mod tests {
    use cosmwasm_std::Uint128;
    
    #[test]
    fn test_simple_addition() {
        let a = Uint128::new(100);
        let b = Uint128::new(200);
        assert_eq!(a + b, Uint128::new(300));
    }
    
    #[test]
    fn test_simple_subtraction() {
        let a = Uint128::new(200);
        let b = Uint128::new(100);
        assert_eq!(a - b, Uint128::new(100));
    }
    
    #[test]
    fn test_simple_multiplication() {
        let a = Uint128::new(10);
        let b = Uint128::new(20);
        assert_eq!(a * b, Uint128::new(200));
    }
}
