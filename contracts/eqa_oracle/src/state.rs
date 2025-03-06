use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub price_timeout: u64,  // in seconds
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct RegisteredAsset {
    pub denom: String,
    pub symbol: String, // Human-readable symbol
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const REGISTERED_ASSETS: Map<&str, RegisteredAsset> = Map::new("registered_assets");
