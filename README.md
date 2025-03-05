# Equilibria (EQA) Smart Contracts

This repository contains the complete implementation of the Equilibria (EQA) stablecoin system for Terra blockchain.

## Components

- **EQA Token Contract**: Handles minting & redemption with dynamic liquidity fees
- **Collateral Manager**: Manages USDC & other collateral reserves
- **Liquidation Engine**: Handles automatic liquidation of assets if under-collateralization occurs
- **Arbitrage Module**: Incentivizes peg stability by rewarding profitable arbitrage trades
- **Governance**: Handles DAO-based decision-making & upgrades

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
