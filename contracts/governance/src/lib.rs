use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Addr, Uint128,
};
use equilibria_smart_contracts::error::ContractError;

mod contract;
mod state;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    contract::initialize(deps, info, msg.voting_period, msg.quorum_percentage)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ProposeUpgrade { title, description, contract_addr, new_code_id } => 
            contract::execute_propose(deps, env, info, title, description, contract_addr, new_code_id),
        ExecuteMsg::Vote { proposal_id, vote } => 
            contract::execute_vote(deps, env, info, proposal_id, vote),
        ExecuteMsg::ExecuteProposal { proposal_id } => 
            contract::execute_proposal(deps, env, info, proposal_id),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetProposal { id } => 
            to_binary(&contract::query_proposal(deps, id)?),
        QueryMsg::ListProposals { start_after, limit } => 
            to_binary(&contract::list_proposals(deps, start_after, limit)?),
    }
}

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub voting_period: u64, // in seconds
    pub quorum_percentage: u64, // percentage (1-100)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ProposeUpgrade { 
        title: String,
        description: String,
        contract_addr: Addr,
        new_code_id: u64,
    },
    Vote { 
        proposal_id: u64,
        vote: VoteOption,
    },
    ExecuteProposal { 
        proposal_id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetProposal { id: u64 },
    ListProposals { 
        start_after: Option<u64>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum VoteOption {
    Yes,
    No,
    Abstain,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "lowercase")]
pub enum ProposalStatus {
    Pending,
    Passed,
    Rejected,
    Executed,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ProposalResponse {
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
pub struct ProposalListResponse {
    pub proposals: Vec<ProposalResponse>,
}
