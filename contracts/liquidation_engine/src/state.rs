use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub oracle_address: Option<Addr>, // Address of the price oracle
    pub threshold_ratio: u64,        // Minimum collateralization ratio (e.g. 110%)
    pub liquidation_fee: u64,        // Fee charged during liquidation (e.g. 5%)
    pub is_active: bool,             // Can be deactivated in emergency
}

pub const CONFIG: Item<Config> = Item::new("config");
