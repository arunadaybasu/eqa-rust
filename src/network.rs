use cosmwasm_std::{Deps, DepsMut, Env, StdError, StdResult};
use crate::config::{NetworkEnvironment, storage::GLOBAL_CONFIG};

/// Network detection and management functions
pub struct NetworkManager {}

impl NetworkManager {
    /// Detect the current network from the chain_id in the environment
    pub fn detect_network(env: &Env) -> NetworkEnvironment {
        let chain_id = &env.block.chain_id;
        
        // Check for mainnet chain ids
        if chain_id == "phoenix-1" || chain_id == "columbus-5" {
            return NetworkEnvironment::Mainnet;
        }
        
        // Check for testnet chain ids
        if chain_id == "pisco-1" || chain_id.starts_with("bombay") {
            return NetworkEnvironment::Testnet;
        }
        
        // Default to localnet for any other chain id
        NetworkEnvironment::LocalNet
    }
    
    /// Verify that we're running in the expected network environment
    pub fn verify_network(
        deps: Deps, 
        env: &Env, 
        expected: NetworkEnvironment,
    ) -> StdResult<()> {
        let current = Self::detect_network(env);
        
        // Get saved configuration
        let config = GLOBAL_CONFIG.load(deps.storage)?;
        
        // If fallback is disabled, enforce strict network matching
        if !config.fallback_enabled && current != expected {
            return Err(StdError::generic_err(format!(
                "Network mismatch: expected {:?}, got {:?}", expected, current
            )));
        }
        
        Ok(())
    }
    
    /// Automatically set the active network based on environment
    pub fn auto_configure_network(
        deps: DepsMut,
        env: &Env,
    ) -> StdResult<NetworkEnvironment> {
        let detected = Self::detect_network(env);
        
        // Update the config to use the detected network
        GLOBAL_CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
            config.active_network = detected.clone();
            Ok(config)
        })?;
        
        Ok(detected)
    }
    
    /// Get the appropriate address for a contract based on current network
    pub fn get_contract_address(
        deps: Deps,
        contract_name: &str,
    ) -> StdResult<String> {
        let config = GLOBAL_CONFIG.load(deps.storage)?;
        let active_config = config.get_active_config();
        
        // In a real implementation, this would query the registry
        // For this example, we're just returning sample addresses based on contract name
        match contract_name {
            "registry" => Ok(active_config.registry_address.clone()),
            "oracle" => Ok(active_config.oracle_address.clone()),
            "fee_collector" => Ok(active_config.fee_collector_address.clone()),
            "axelar_gateway" => Ok(active_config.gateway_config.axelar_gateway.clone()),
            "noble_gateway" => Ok(active_config.gateway_config.noble_gateway.clone()),
            _ => Err(StdError::generic_err(format!("Unknown contract: {}", contract_name))),
        }
    }
}
