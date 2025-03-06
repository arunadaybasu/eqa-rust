# Terra Classic Oracle Integration for Equilibria

This document outlines the integration between Equilibria (EQA) and the Terra Classic Oracle.

## Architecture Overview

```
┌───────────────┐         ┌────────────────┐         ┌─────────────────┐
│  EQA Token    │         │  EQA Oracle    │         │  Terra Classic  │
│  Contract     │◄───────►│  Contract      │◄───────►│  Oracle Module  │
└───────────────┘         └────────────────┘         └─────────────────┘
        │                                                      │
        │                                                      │
        ▼                                                      ▼
┌───────────────┐                                    ┌─────────────────┐
│  Collateral   │                                    │    External     │
│  Manager      │                                    │  Price Sources  │
└───────────────┘                                    └─────────────────┘
```

## Oracle Implementation

The Oracle integration consists of two main components:

1. The EQA Oracle contract that wraps the native Terra Classic Oracle
2. Integration points in the EQA Token and Collateral Manager contracts

### EQA Oracle Contract

This smart contract provides a standardized interface to access the Terra Classic Oracle. It:

- Queries the Terra Classic Oracle using the `TerraQuerier` from `terra-cosmwasm`
- Maintains a registry of supported assets
- Provides price data for EQA and other assets
- Handles exchange rate calculations

### Integration with EQA Token Contract

The EQA token contract is updated to:

1. Reference the EQA Oracle for real-time price data
2. Use this data to calculate dynamic fees
3. Make informed decisions about minting and redemption

## Usage Examples

### Querying the Oracle

```rust
// Get EQA price
let query_msg = QueryMsg::GetPrice { denom: "eqa".to_string() };
let price_response: PriceResponse = deps.querier.query_wasm_smart(
    oracle_address,
    &query_msg
)?;

// Get exchange rate between EQA and LUNA
let query_msg = QueryMsg::GetExchangeRate { 
    base_denom: "eqa".to_string(),
    quote_denom: "uluna".to_string()
};
let rate_response: ExchangeRateResponse = deps.querier.query_wasm_smart(
    oracle_address,
    &query_msg
)?;
```

### Dynamic Fee Calculation

```rust
// Get current market price from oracle
let price_query = QueryMsg::GetPrice { denom: "eqa".to_string() };
let price_response: PriceResponse = deps.querier.query_wasm_smart(
    oracle_address,
    &price_query
)?;

// Calculate fee based on price deviation
let fee = calculate_dynamic_fee(price_response.price);
let fee_amount = amount * fee;
```

## Benefits of Oracle Integration

1. **Real-time Price Data**: Always use the most current market prices
2. **Dynamic Fee Mechanism**: Adjust fees automatically based on market conditions
3. **Arbitrage Incentives**: Create profitable arbitrage opportunities when EQA deviates from peg
4. **Security**: Reliance on multiple price sources through Terra Classic Oracle
5. **Transparency**: Clear mechanism for determining exchange rates

## Technical Notes

- The Terra Classic Oracle samples prices from multiple sources, increasing reliability
- Price timeouts can be configured to ensure stale prices are not used
- External integrations can verify prices independently
