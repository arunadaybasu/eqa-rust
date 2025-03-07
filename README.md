# Equilibria (EQA) Smart Contracts

This repository contains the complete implementation of the Equilibria (EQA) stablecoin system for Terra blockchain.

## Project Structure

```
equilibria-smart-contracts/
├── src/
│   ├── lib.rs        # Main library exports
│   ├── error.rs      # Error handling
│   └── state.rs      # State management
├── contracts/
│   ├── eqa_token/            # EQA token implementation
│   │   ├── src/
│   │   │   ├── lib.rs        # Entry points and messages
│   │   │   └── contract.rs   # Business logic
│   │   └── Cargo.toml
│   ├── collateral_manager/   # Collateral management
│   │   ├── src/
│   │   │   ├── lib.rs        # Entry points and messages
│   │   │   └── contract.rs   # Business logic
│   │   └── Cargo.toml
│   ├── liquidation_engine/   # Liquidation handling
│   │   ├── src/
│   │   │   ├── lib.rs        # Entry points and messages
│   │   │   └── contract.rs   # Business logic
│   │   └── Cargo.toml
│   ├── arbitrage_module/     # Arbitrage incentives
│   │   ├── src/
│   │   │   ├── lib.rs        # Entry points and messages
│   │   │   └── contract.rs   # Business logic
│   │   └── Cargo.toml
│   └── governance/           # DAO governance
│       ├── src/
│       │   ├── lib.rs        # Entry points and messages
│       │   ├── contract.rs   # Business logic
│       │   └── state.rs      # Governance state
│       └── Cargo.toml
└── Cargo.toml
```

## Components

### Core Library
- **State Management**: Token and collateral state with CosmWasm storage interfaces
- **Error Handling**: Custom error types for the entire system

### EQA Token Contract
- **Minting**: Create new EQA tokens with dynamic fee calculation based on market price
- **Redemption**: Redeem EQA tokens for collateral with dynamic fee adjustment

### Collateral Manager
- **Reserve Management**: Manages USDC from multiple sources (Axelar, Noble)
- **Total Locked Value**: Tracks the total collateral backing the system

### Liquidation Engine
- **Collateralization Monitoring**: Checks if the system is properly collateralized
- **Liquidation Triggers**: Executes liquidations when collateral ratio falls below threshold
- **Status Reporting**: Provides system solvency information

### Arbitrage Module
- **Market Monitoring**: Detects arbitrage opportunities based on price deviation
- **Incentive Calculation**: Calculates rewards for arbitrageurs that help maintain the peg
- **Trade Execution**: Handles the execution of profitable arbitrage trades

### Governance
- **Proposal Management**: Create, vote on, and execute governance proposals
- **Voting System**: Weighted voting based on token holdings
- **Contract Upgrades**: Migration and upgrade management for the protocol

## Deployment Instructions

1. Compile Contracts:
```bash
cargo wasm
```

2. Deploy to Terra Classic:
```bash
terrad tx wasm store equilibria_smart_contracts.wasm --from mywallet --gas auto --fees 100uluna
```

3. Execute Mint Function:
```bash
terrad tx wasm execute <CONTRACT_ADDRESS> '{"mint": {"amount":"100", "market_price":"1.02"}}' --from mywallet --gas auto --fees 50uluna
```

## Key Features

- **Dynamic Fee System**: Fees adjust based on market conditions to incentivize peg stability
- **Multi-Source Collateral**: Support for multiple USDC sources to diversify risk
- **Automated Liquidations**: System monitors and maintains required collateralization
- **Decentralized Governance**: DAO-based protocol management
- **Arbitrage Incentives**: Built-in mechanisms to reward arbitrageurs who help maintain the peg
