use cosmwasm_std::{DepsMut, Deps, Env, MessageInfo, Response, StdResult, StdError, Addr, Decimal};
use terra_cosmwasm::TerraQuerier;
use equilibria_smart_contracts::error::ContractError;
use crate::state::{CONFIG, REGISTERED_ASSETS, Config, RegisteredAsset};
use crate::{PriceResponse, ExchangeRateResponse, RegisteredAssetResponse};

pub fn initialize(
    deps: DepsMut,
    _info: MessageInfo,
    admin: Addr,
    price_timeout: u64,
) -> Result<Response, ContractError> {
    let config = Config {
        admin,
        price_timeout,
    };
    CONFIG.save(deps.storage, &config)?;

    // Register default assets (Luna and USD)
    let luna_asset = RegisteredAsset {
        denom: "uluna".to_string(),
        symbol: "LUNA".to_string(),
    };
    REGISTERED_ASSETS.save(deps.storage, "uluna", &luna_asset)?;

    let usd_asset = RegisteredAsset {
        denom: "uusd".to_string(),
        symbol: "UST".to_string(),
    };
    REGISTERED_ASSETS.save(deps.storage, "uusd", &usd_asset)?;

    Ok(Response::new()
        .add_attribute("action", "initialize")
        .add_attribute("admin", admin.to_string())
        .add_attribute("price_timeout", price_timeout.to_string()))
}

pub fn update_admin(
    deps: DepsMut,
    info: MessageInfo,
    new_admin: Addr,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    
    // Check if the sender is the current admin
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }
    
    config.admin = new_admin.clone();
    CONFIG.save(deps.storage, &config)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_admin")
        .add_attribute("new_admin", new_admin.to_string()))
}

pub fn update_price_timeout(
    deps: DepsMut,
    info: MessageInfo,
    new_timeout: u64,
) -> Result<Response, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;
    
    // Check if the sender is the current admin
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }
    
    config.price_timeout = new_timeout;
    CONFIG.save(deps.storage, &config)?;
    
    Ok(Response::new()
        .add_attribute("action", "update_price_timeout")
        .add_attribute("new_timeout", new_timeout.to_string()))
}

pub fn register_asset(
    deps: DepsMut,
    info: MessageInfo,
    denom: String,
    symbol: String,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    
    // Check if the sender is the current admin
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }
    
    let asset = RegisteredAsset {
        denom: denom.clone(),
        symbol: symbol.clone(),
    };
    
    REGISTERED_ASSETS.save(deps.storage, &denom, &asset)?;
    
    Ok(Response::new()
        .add_attribute("action", "register_asset")
        .add_attribute("denom", denom)
        .add_attribute("symbol", symbol))
}

pub fn query_price(
    deps: Deps,
    env: Env,
    denom: String,
) -> StdResult<PriceResponse> {
    // Check if the asset is registered
    let _asset = REGISTERED_ASSETS.load(deps.storage, &denom)?;
    
    // Query Terra Classic oracle for price
    let querier = TerraQuerier::new(&deps.querier);
    let exchange_rate = querier.query_exchange_rate(denom.clone(), "uusd".to_string())?;
    
    // Convert to Decimal format
    let price = Decimal::from_ratio(exchange_rate.exchange_rate.numerator(), exchange_rate.exchange_rate.denominator());
    
    Ok(PriceResponse {
        denom,
        price,
        last_updated: env.block.time.seconds(),
    })
}

pub fn query_exchange_rate(
    deps: Deps,
    env: Env,
    base_denom: String,
    quote_denom: String,
) -> StdResult<ExchangeRateResponse> {
    // Check if both assets are registered
    let _base_asset = REGISTERED_ASSETS.load(deps.storage, &base_denom)?;
    let _quote_asset = REGISTERED_ASSETS.load(deps.storage, &quote_denom)?;
    
    // Query Terra Classic oracle for exchange rate
    let querier = TerraQuerier::new(&deps.querier);
    let exchange_rate = querier.query_exchange_rate(base_denom.clone(), quote_denom.clone())?;
    
    // Convert to Decimal format
    let rate = Decimal::from_ratio(exchange_rate.exchange_rate.numerator(), exchange_rate.exchange_rate.denominator());
    
    Ok(ExchangeRateResponse {
        base_denom,
        quote_denom,
        rate,
        last_updated: env.block.time.seconds(),
    })
}

pub fn query_registered_assets(deps: Deps) -> StdResult<RegisteredAssetResponse> {
    let assets: Vec<RegisteredAsset> = REGISTERED_ASSETS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|item| {
            let (_, asset) = item?;
            Ok(RegisteredAsset {
                denom: asset.denom,
                symbol: asset.symbol,
            })
        })
        .collect::<StdResult<_>>()?;
    
    Ok(RegisteredAssetResponse { assets })
}
