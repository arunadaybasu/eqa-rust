# Contract Registry

This contract serves as a central registry for all contract addresses in the Equilibria ecosystem. Instead of hardcoding addresses in each contract, we use this registry to store and retrieve addresses dynamically.

## Key Features

- Centralized address management
- Admin-controlled updates
- Query interface for contracts to retrieve addresses

## Usage

### Instantiate

```rust
let msg = InstantiateMsg {
    admin: Option<Addr>, // Optional admin address, defaults to sender
};
```

### Execute Messages

Set a contract address:
```rust
let msg = ExecuteMsg::SetContractAddress { 
    name: "axelar_usdc".to_string(), 
    address: "terra1...".to_string() 
};
```

Update admin:
```rust
let msg = ExecuteMsg::UpdateConfig { 
    new_admin: Some(Addr::unchecked("new_admin")) 
};
```

### Query Messages

Get a specific contract address:
```rust
let msg = QueryMsg::GetContractAddress { 
    name: "axelar_usdc".to_string() 
};
```

Get all registered contracts:
```rust
let msg = QueryMsg::GetAllContracts {};
```

## Integration with Other Contracts

Other contracts should store the registry address and query it for needed addresses instead of hardcoding them:

```rust
// Store registry address during instantiation
pub const REGISTRY_ADDRESS: Item<String> = Item::new("registry_address");

// Query address from registry
fn get_contract_address(deps: Deps, registry_addr: &str, contract_key: &str) -> StdResult<String> {
    let query_msg = registry::QueryMsg::GetContractAddress {
        name: contract_key.to_string(),
    };
    
    let query = QueryRequest::Wasm(WasmQuery::Smart {
        contract_addr: registry_addr.to_string(),
        msg: to_json_binary(&query_msg)?,
    });
    
    let response: registry::ContractAddressResponse = deps.querier.query(&query)?;
    Ok(response.address)
}
```
