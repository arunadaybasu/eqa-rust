
#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{from_binary, Uint128};

    #[test]
    fn test_mint_eqa() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info("sender", &[]);

        let res = execute_mint(deps.as_mut(), env, info, Uint128::new(100));
        assert!(res.is_ok());
    }
}
