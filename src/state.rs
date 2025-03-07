use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::{Item, Map};

/// State struct to track EQA token 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TokenState {
    pub total_supply: Uint128, // Total EQA supply
    pub owner: Addr,
    pub name: String,          // Token name
    pub symbol: String,        // Token symbol
    pub decimals: u8,          // Decimal precision
}

/// State struct to track collateral amounts
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CollateralState {
    pub usdc_axelar: Uint128, // USDC from Axelar
    pub usdc_noble: Uint128,  // USDC from Noble
    pub total_locked: Uint128, // Total collateral value
}

/// Item to track token state
pub const TOKEN_STATE: Item<TokenState> = Item::new("token_state");
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balances");

/// Item to track the collateral state
pub const COLLATERAL: Item<CollateralState> = Item::new("collateral_state");
