
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
    use crate::error::ContractError;
    use crate::state::{TOKEN_STATE, BALANCES};

    pub fn execute_mint(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        let mut state = TOKEN_STATE.load(deps.storage)?;
        
        state.total_supply += amount;
        BALANCES.update(deps.storage, info.sender.clone(), |bal| -> StdResult<_> {
            Ok(bal.unwrap_or_default() + amount)
        })?;

        Ok(Response::new().add_attribute("minted", amount.to_string()))
    }
    