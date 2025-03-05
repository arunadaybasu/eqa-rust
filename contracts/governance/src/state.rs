use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cw_storage_plus::{Item, Map};
use crate::{ProposalStatus, VoteOption};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub voting_period: u64, // in seconds
    pub quorum_percentage: u64, // percentage (1-100)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Proposal {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub status: ProposalStatus,
    pub contract_addr: Addr,
    pub new_code_id: u64,
    pub yes_votes: Uint128,
    pub no_votes: Uint128,
    pub abstain_votes: Uint128,
    pub end_time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Vote {
    pub voter: Addr,
    pub proposal_id: u64,
    pub vote: VoteOption,
    pub voting_power: Uint128,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const PROPOSAL_COUNT: Item<u64> = Item::new("proposal_count");
pub const PROPOSALS: Map<u64, Proposal> = Map::new("proposals");
pub const VOTES: Map<(u64, &Addr), Vote> = Map::new("votes"); // (proposal_id, voter) -> Vote
