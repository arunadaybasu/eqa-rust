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


# Equilibria (EQA) Smart Contract System: Implementation Overview

## Currently Implemented Components

### Core Contracts

1. **EQA Token Contract**
   - Basic ERC20-like functionality
   - Dynamic fee mechanism based on price deviation from peg
   - Integration with Oracle for price feeds
   - Minting and redemption functionality

2. **Collateral Manager**
   - Storage and tracking of collateral assets (USDC from Axelar and Noble)
   - Collateral update functionality
   - Total collateral calculation

3. **Liquidation Engine**
   - Solvency checks based on collateral ratio
   - Integration with Oracle for price feeds
   - Configurable threshold ratio for minimum collateralization
   - Emergency pause functionality

4. **Oracle Implementation**
   - Integration with Terra Classic Oracle
   - Price data retrieval for EQA and other assets
   - Exchange rate calculations
   - Asset registration system

5. **Arbitrage Module**
   - Opportunity identification when price deviates from peg
   - Incentive mechanism through rewards
   - Interface for executing arbitrage operations

### Supporting Components

1. **Shared Libraries**
   - Error handling
   - State management
   - Oracle interface definitions
   - Mock implementations for testing

2. **Tests**
   - Unit tests for math operations
   - Integration tests for contract interactions
   - Mock testing infrastructure

## What Needs to Be Implemented

### Core Functionality

1. **Governance Contract**
   - Implementation of governance proposals
   - Voting mechanism
   - Parameter updates (fees, thresholds, etc.)
   - Emergency controls

2. **Complete Liquidation Logic**
   - Actual asset liquidation process
   - Liquidation auction mechanism
   - Disbursement of liquidated assets

3. **External Asset Integration**
   - Complete CW20 token handling for USDC
   - Integration with Axelar and Noble bridges
   - Cross-chain message handling

### User Interactions

1. **Frontend Integration**
   - Query handlers for UI data
   - Transaction formatting
   - User balance and position monitoring

2. **Complete Minting Flow**
   - Direct deposit of collateral when minting
   - Receipt handling
   - Fee collection mechanism

3. **Complete Redemption Flow**
   - Collateral release during redemption
   - Fee application
   - Oracle price verification

### Security & Operations

1. **Access Control System**
   - Role-based permissions
   - Multi-signature operation for critical functions
   - Timelocks for parameter changes

2. **Complete Error Handling**
   - More comprehensive error messages
   - Recovery mechanisms
   - Sanitized inputs

3. **Deployment & Upgrade System**
   - Contract upgrade mechanism
   - Migration data handling
   - Versioning system

4. **Monitoring & Analytics**
   - On-chain analytics
   - Health metrics
   - Risk indicators

## Technical Details

### Implemented Libraries and Patterns

- **Storage**: Using `cw-storage-plus` for efficient data storage
- **Types**: Proper handling of `Uint128` and `Decimal` for financial calculations
- **Error Handling**: Custom error types for detailed feedback
- **Testing**: Unit and integration testing approaches

### Deployment Architecture

The system is structured as a workspace with multiple contracts:
```
workspace
├── contracts/
│   ├── eqa_token
│   ├── collateral_manager
│   ├── liquidation_engine
│   ├── arbitrage_module
│   ├── eqa_oracle
│   └── governance (not yet implemented)
└── src/ (shared libraries)
    ├── error.rs
    ├── state.rs
    ├── oracle.rs
    └── mocks.rs
```

## Next Steps and Priorities

1. **Complete the Governance Contract**
   - This is critical for decentralized control of the protocol

2. **Finish Liquidation Implementation**
   - Essential for maintaining system solvency

3. **Expand Test Coverage**
   - More comprehensive tests, especially for edge cases and security scenarios

4. **Implement Proper Asset Bridges**
   - Required for handling real USDC across chains

5. **Security Audits**
   - Critical before any production deployment

6. **Documentation**
   - Complete API documentation
   - Integration guides
   - Operational procedures

## Development Guidelines

- Maintain consistent error handling patterns
- Ensure all financial calculations use safe math operations
- Document all public functions and state variables
- Add tests for new functionality
- Follow the existing architecture patterns for consistency

The system has a solid foundation with the core contracts and shared infrastructure in place, but requires additional work on the governance, security, and integration aspects to be production-ready.
