# Running and Testing Equilibria Smart Contracts

This guide provides instructions for building, testing, and deploying the Equilibria (EQA) smart contracts.

## Prerequisites

1. **Install Rust and Cargo**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Add WebAssembly target**
   ```bash
   rustup target add wasm32-unknown-unknown
   ```

3. **Install cargo-generate**
   ```bash
   cargo install cargo-generate
   ```

4. **Install cosmwasm-check**
   ```bash
   cargo install cosmwasm-check
   ```

## Building the Contracts

Build all contracts in the workspace:

```bash
cd /c:/projects-terra/eqa-rust
cargo build
```

To build for production (optimized WASM files):

```bash
cargo build --target wasm32-unknown-unknown --release
```

## Running Unit Tests

Run all tests in the workspace:

```bash
cargo test
```

Test a specific contract:

```bash
cargo test -p eqa_token
cargo test -p collateral_manager
cargo test -p liquidation_engine
cargo test -p arbitrage_module
cargo test -p governance
```

## Creating Integration Tests

Create a comprehensive integration test in the `/c:/projects-terra/eqa-rust/tests` directory:

```rust
// Example integration test structure
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, Addr, Uint128, Decimal};

use eqa_token::contract::{execute_mint, execute_redeem};
use eqa_token::msg::{ExecuteMsg, InstantiateMsg};
use collateral_manager::contract::execute_update_collateral;

#[test]
fn test_full_flow() {
    // Setup
    let mut deps = mock_dependencies();
    let env = mock_env();
    
    // Initialize contracts
    
    // Test minting with collateral
    
    // Test redemption
    
    // Test liquidation when under-collateralized
}
```

## Using Terra LocalTerra for Local Testing

1. **Setup LocalTerra**
   ```bash
   git clone https://github.com/terra-money/LocalTerra
   cd LocalTerra
   docker-compose up
   ```

2. **Deploy to LocalTerra**
   ```bash
   terrad tx wasm store artifacts/eqa_token.wasm --from test1 --chain-id=localterra --gas=auto --fees=100000uluna
   ```

3. **Instantiate Contract**
   ```bash
   terrad tx wasm instantiate 1 '{"name":"Equilibria","symbol":"EQA","decimals":6}' --from test1 --chain-id=localterra --label="EQA Token" --gas=auto --fees=100000uluna
   ```

## Using CosmWasm Multi-Test for Integration Testing

CosmWasm's multi-test framework allows testing interactions between multiple contracts:

```rust
use cosmwasm_std::{Addr, Coin, Empty};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};

fn eqa_token_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(
        eqa_token::contract::execute,
        eqa_token::contract::instantiate,
        eqa_token::contract::query,
    );
    Box::new(contract)
}

#[test]
fn test_end_to_end() {
    // Setup test application
    let mut app = App::default();
    
    // Store contracts
    let eqa_code_id = app.store_code(eqa_token_contract());
    let collateral_code_id = app.store_code(collateral_manager_contract());
    
    // Instantiate contracts
    
    // Test interactions between contracts
}
```

## Creating a GitHub Actions Pipeline

Create a GitHub Actions workflow to automatically build and test your contracts:

```yaml
name: Build and Test

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v2
    - name: Install latest rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: wasm32-unknown-unknown
        override: true
    - name: Build
      run: |
        cargo build --verbose
    - name: Run tests
      run: |
        cargo test --verbose
```

## Deploying to TestNet

1. **Build optimized contract**
   ```bash
   RUSTFLAGS='-C link-arg=-s' cargo wasm
   ```

2. **Deploy to Terra Testnet**
   ```bash
   terrad tx wasm store artifacts/eqa_token.wasm --from mywallet --chain-id=pisco-1 --gas=auto --fees=100000uluna
   ```

3. **Instantiate your contract**
   ```bash
   terrad tx wasm instantiate $CODE_ID '{"name":"Equilibria","symbol":"EQA","decimals":6}' --from mywallet --chain-id=pisco-1 --label="EQA Token" --gas=auto --fees=100000uluna
   ```

4. **Interact with your contract**
   ```bash
   terrad tx wasm execute $CONTRACT_ADDR '{"mint":{"amount":"1000000","market_price":"1.0"}}' --from mywallet --chain-id=pisco-1 --gas=auto --fees=100000uluna
   ```
