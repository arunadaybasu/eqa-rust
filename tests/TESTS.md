# Running Tests for Equilibria EQA

This document explains how to run tests for the EQA smart contracts.

## Basic Testing

Run all tests:
```bash
cargo test
```

Run a specific test:
```bash
cargo test test_basic_calculations
```

Run tests only for a specific module:
```bash
cargo test --test basic_tests
```

## Test Structure

The tests are organized as follows:

1. `basic_tests.rs` - Simple unit tests for basic functionality
2. `integration_test.rs` - Tests that interact with contract storage
3. `test_eqa.rs` - More comprehensive tests for EQA token functionality

## Common Issues

If you encounter test failures, check for:

1. **Type errors** - Make sure you're using the right types (e.g., `u64` instead of floating-point numbers)
2. **Missing dependencies** - Ensure all required crates are included
3. **Storage issues** - When using `mock_dependencies()`, make sure you're properly managing storage

## Adding New Tests

When adding new tests, follow this pattern:

```rust
#[cfg(test)]
mod tests {
    use cosmwasm_std::{Uint128, Decimal};
    
    #[test]
    fn test_your_feature() {
        // Test setup
        
        // Action
        
        // Assertions
        assert_eq!(expected, actual);
    }
}
```
