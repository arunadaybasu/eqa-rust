
    use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Uint128};
    use crate::error::ContractError;
    use crate::state::COLLATERAL;

    pub fn execute_update_collateral(
        deps: DepsMut,
        _env: Env,
        info: MessageInfo,
        usdc_axelar: Uint128,
        usdc_noble: Uint128,
    ) -> Result<Response, ContractError> {
        let mut state = COLLATERAL.load(deps.storage)?;

        state.usdc_axelar = usdc_axelar;
        state.usdc_noble = usdc_noble;
        state.total_locked = usdc_axelar + usdc_noble;

        COLLATERAL.save(deps.storage, &state)?;

        Ok(Response::new().add_attribute("updated_collateral", state.total_locked.to_string()))
    }
    