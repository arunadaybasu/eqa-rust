use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub oracle_address: Option<Addr>, // Address of the EQA oracle
}

pub const CONFIG: Item<Config> = Item::new("config");
