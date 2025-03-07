use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MinterData {
    pub minter: Addr,
    pub cap: Option<Uint128>,
    pub price_feed: Option<String>,
    pub collateral_denom: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenState {
    pub total_supply: Uint128,
    pub fee_accumulated: Uint128,
    pub last_action_block: u64,
}

// Store token info like name, symbol, decimals
pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");

// Store total token supply
pub const TOKEN_SUPPLY: Item<Uint128> = Item::new("token_supply");

// Store minter data
pub const MINTER: Item<Option<MinterData>> = Item::new("minter");

// Store user balances
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balances");

// Store token state for more complex data
pub const TOKEN_STATE: Item<TokenState> = Item::new("token_state");

// Config for dynamic aspects
pub const CONFIG: Item<Config> = Item::new("config");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub fee_collector: Addr,
    pub max_fee_percentage: u64, // in basis points (e.g., 500 = 5%)
    pub min_fee_percentage: u64, // in basis points (e.g., 10 = 0.1%)
    pub oracle_address: String,
}
