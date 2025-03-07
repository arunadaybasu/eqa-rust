use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const ADMIN: Item<Addr> = Item::new("admin");
pub const REGISTRY_ADDRESS: Item<String> = Item::new("registry_address");
