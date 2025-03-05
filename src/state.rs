use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Storage;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenState {
    pub total_supply: Uint128,
    pub owner: Addr,
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralState {
    pub usdc_axelar: Uint128,
    pub usdc_noble: Uint128,
    pub total_locked: Uint128,
}

pub const TOKEN_STATE: Item<TokenState> = Item::new("token_state");
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balances");
pub const COLLATERAL: Item<CollateralState> = Item::new("collateral");
