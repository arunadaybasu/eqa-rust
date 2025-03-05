use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, Response, StdResult, Addr, Uint128, Order};
use equilibria_smart_contracts::error::ContractError;

use crate::state::{CONFIG, PROPOSAL_COUNT, PROPOSALS, VOTES, Config, Proposal, Vote};
use crate::{ProposalStatus, VoteOption, ProposalResponse, ProposalListResponse};

pub fn initialize(
    deps: DepsMut,
    info: MessageInfo,
    voting_period: u64,
    quorum_percentage: u64,
) -> Result<Response, ContractError> {
    if quorum_percentage > 100 || quorum_percentage == 0 {
        return Err(ContractError::CustomError { 
            msg: "Quorum percentage must be between 1-100".to_string() 
        });
    }

    let config = Config {
        admin: info.sender.clone(),
        voting_period,
        quorum_percentage,
    };
    
    CONFIG.save(deps.storage, &config)?;
    PROPOSAL_COUNT.save(deps.storage, &0u64)?;
    
    Ok(Response::new()
        .add_attribute("action", "initialize")
        .add_attribute("admin", info.sender)
        .add_attribute("voting_period", voting_period.to_string())
        .add_attribute("quorum_percentage", quorum_percentage.to_string()))
}

pub fn execute_propose(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    title: String,
    description: String,
    contract_addr: Addr,
    new_code_id: u64,
) -> Result<Response, ContractError> {
    // In a real implementation, you would check if sender has enough voting power
    
    let config = CONFIG.load(deps.storage)?;
    let proposal_id = PROPOSAL_COUNT.load(deps.storage)? + 1;
    PROPOSAL_COUNT.save(deps.storage, &proposal_id)?;
    
    let proposal = Proposal {
        id: proposal_id,
        title,
        description,
        status: ProposalStatus::Pending,
        contract_addr,
        new_code_id,
        yes_votes: Uint128::zero(),
        no_votes: Uint128::zero(),
        abstain_votes: Uint128::zero(),
        end_time: env.block.time.seconds() + config.voting_period,
    };
    
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;
    
    Ok(Response::new()
        .add_attribute("action", "propose")
        .add_attribute("creator", info.sender)
        .add_attribute("proposal_id", proposal_id.to_string()))
}

pub fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    proposal_id: u64,
    vote: VoteOption,
) -> Result<Response, ContractError> {
    let proposal = PROPOSALS.load(deps.storage, proposal_id)?;
    
    // Check if proposal is still active
    if env.block.time.seconds() > proposal.end_time {
        return Err(ContractError::CustomError {
            msg: "Voting period has ended".to_string(),
        });
    }
    
    if proposal.status != ProposalStatus::Pending {
        return Err(ContractError::CustomError {
            msg: "Proposal is not in voting period".to_string(),
        });
    }
    
    // In a real implementation, you would calculate voting power based on token balance
    let voting_power = Uint128::from(1u128); // Simple 1 vote per address
    
    // Record the vote
    let vote_record = Vote {
        voter: info.sender.clone(),
        proposal_id,
        vote: vote.clone(),
        voting_power,
    };
    
    VOTES.save(deps.storage, (proposal_id, &info.sender), &vote_record)?;
    
    // Update proposal vote counts
    let mut updated_proposal = proposal;
    match vote {
        VoteOption::Yes => updated_proposal.yes_votes += voting_power,
        VoteOption::No => updated_proposal.no_votes += voting_power,
        VoteOption::Abstain => updated_proposal.abstain_votes += voting_power,
    }
    
    PROPOSALS.save(deps.storage, proposal_id, &updated_proposal)?;
    
    Ok(Response::new()
        .add_attribute("action", "vote")
        .add_attribute("voter", info.sender)
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("vote", format!("{:?}", vote)))
}

pub fn execute_proposal(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    proposal_id: u64,
) -> Result<Response, ContractError> {
    let mut proposal = PROPOSALS.load(deps.storage, proposal_id)?;
    let config = CONFIG.load(deps.storage)?;
    
    // Check if voting period has ended
    if env.block.time.seconds() <= proposal.end_time {
        return Err(ContractError::CustomError {
            msg: "Voting period is not over yet".to_string(),
        });
    }
    
    // Only admin can execute proposals for safety
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }
    
    // Calculate total votes and check quorum
    let total_votes = proposal.yes_votes + proposal.no_votes + proposal.abstain_votes;
    let quorum = Uint128::from(100u128); // In a real implementation, this would be from total supply
    let quorum_reached = total_votes.u128() * 100 >= quorum.u128() * config.quorum_percentage as u128;
    
    if !quorum_reached {
        proposal.status = ProposalStatus::Rejected;
        PROPOSALS.save(deps.storage, proposal_id, &proposal)?;
        
        return Err(ContractError::CustomError {
            msg: "Quorum not reached".to_string(),
        });
    }
    
    // Check if proposal has passed
    let passed = proposal.yes_votes > proposal.no_votes;
    
    if passed {
        proposal.status = ProposalStatus::Executed;
        
        // In a real implementation, this would actually execute the upgrade
        // through a MsgMigrateContract or similar
    } else {
        proposal.status = ProposalStatus::Rejected;
    }
    
    PROPOSALS.save(deps.storage, proposal_id, &proposal)?;
    
    Ok(Response::new()
        .add_attribute("action", "execute_proposal")
        .add_attribute("proposal_id", proposal_id.to_string())
        .add_attribute("result", if passed { "passed" } else { "rejected" }))
}

pub fn query_proposal(deps: Deps, id: u64) -> StdResult<ProposalResponse> {
    let proposal = PROPOSALS.load(deps.storage, id)?;
    
    Ok(ProposalResponse {
        id: proposal.id,
        title: proposal.title,
        description: proposal.description,
        status: proposal.status,
        contract_addr: proposal.contract_addr,
        new_code_id: proposal.new_code_id,
        yes_votes: proposal.yes_votes,
        no_votes: proposal.no_votes,
        abstain_votes: proposal.abstain_votes,
        end_time: proposal.end_time,
    })
}

pub fn list_proposals(deps: Deps, start_after: Option<u64>, limit: Option<u32>) -> StdResult<ProposalListResponse> {
    let limit = limit.unwrap_or(30) as usize;
    let start = start_after.map(|s| s + 1); // exclusive
    
    let proposal_iter = PROPOSALS
        .range(deps.storage, start, None, Order::Ascending)
        .take(limit);
    
    let mut proposals: Vec<ProposalResponse> = vec![];
    for item in proposal_iter {
        let (_, proposal) = item?;
        proposals.push(ProposalResponse {
            id: proposal.id,
            title: proposal.title,
            description: proposal.description,
            status: proposal.status,
            contract_addr: proposal.contract_addr,
            new_code_id: proposal.new_code_id,
            yes_votes: proposal.yes_votes,
            no_votes: proposal.no_votes,
            abstain_votes: proposal.abstain_votes,
            end_time: proposal.end_time,
        });
    }
    
    Ok(ProposalListResponse { proposals })
}
