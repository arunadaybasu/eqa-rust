use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Api, StdResult};
use std::fmt;

/// Network environments that the contract can operate in
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum NetworkEnvironment {
    Mainnet,
    Testnet,
    LocalNet,
}

impl fmt::Display for NetworkEnvironment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkEnvironment::Mainnet => write!(f, "mainnet"),
            NetworkEnvironment::Testnet => write!(f, "testnet"),
            NetworkEnvironment::LocalNet => write!(f, "localnet"),
        }
    }
}

/// Gateway configuration for cross-chain operations
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GatewayConfig {
    pub axelar_gateway: String,
    pub noble_gateway: String,
}

/// Configuration for different environments
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct NetworkConfig {
    pub environment: NetworkEnvironment,
    pub gateway_config: GatewayConfig,
    pub registry_address: String,
    pub oracle_address: String,
    pub fee_collector_address: String,
}

/// Global configuration including active network and fallback order
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct GlobalConfig {
    pub active_network: NetworkEnvironment,
    pub mainnet_config: NetworkConfig,
    pub testnet_config: NetworkConfig,
    pub localnet_config: NetworkConfig,
    pub fallback_enabled: bool,
}

impl GlobalConfig {
    /// Get the current active configuration, following the fallback chain if enabled
    pub fn get_active_config(&self) -> &NetworkConfig {
        match self.active_network {
            NetworkEnvironment::Mainnet => &self.mainnet_config,
            NetworkEnvironment::Testnet => {
                if self.fallback_enabled && matches!(self.mainnet_config.environment, NetworkEnvironment::Mainnet) {
                    &self.mainnet_config
                } else {
                    &self.testnet_config
                }
            }
            NetworkEnvironment::LocalNet => {
                if self.fallback_enabled {
                    if matches!(self.mainnet_config.environment, NetworkEnvironment::Mainnet) {
                        &self.mainnet_config
                    } else if matches!(self.testnet_config.environment, NetworkEnvironment::Testnet) {
                        &self.testnet_config
                    } else {
                        &self.localnet_config
                    }
                } else {
                    &self.localnet_config
                }
            }
        }
    }
    
    /// Get gateway address for a specific gateway type
    pub fn get_gateway_address(&self, gateway_type: &str) -> String {
        let config = self.get_active_config();
        match gateway_type {
            "axelar" => config.gateway_config.axelar_gateway.clone(),
            "noble" => config.gateway_config.noble_gateway.clone(),
            _ => "".to_string(),
        }
    }
    
    /// Validate addresses in the current active configuration
    pub fn validate_addresses(&self, api: &dyn Api) -> StdResult<()> {
        let config = self.get_active_config();
        
        // Validate registry address
        api.addr_validate(&config.registry_address)?;
        
        // Validate oracle address
        api.addr_validate(&config.oracle_address)?;
        
        // Validate fee collector address
        api.addr_validate(&config.fee_collector_address)?;
        
        // Validate gateway addresses
        api.addr_validate(&config.gateway_config.axelar_gateway)?;
        api.addr_validate(&config.gateway_config.noble_gateway)?;
        
        Ok(())
    }
}

/// Storage configuration for network configs
pub mod storage {
    use super::*;
    use cosmwasm_std::{StdResult, Storage};
    use cw_storage_plus::Item;
    
    // Storage key for global configuration
    pub const GLOBAL_CONFIG: Item<GlobalConfig> = Item::new("global_config");
    
    /// Initialize configuration with default mainnet priority
    pub fn initialize_config(
        storage: &mut dyn Storage,
        mainnet_registry: String,
        testnet_registry: String,
        localnet_registry: String,
    ) -> StdResult<()> {
        let config = GlobalConfig {
            active_network: NetworkEnvironment::Mainnet,
            fallback_enabled: true,
            mainnet_config: NetworkConfig {
                environment: NetworkEnvironment::Mainnet,
                gateway_config: GatewayConfig {
                    axelar_gateway: "terra1axelar_mainnet".to_string(),
                    noble_gateway: "terra1noble_mainnet".to_string(),
                },
                registry_address: mainnet_registry,
                oracle_address: "terra1oracle_mainnet".to_string(),
                fee_collector_address: "terra1fee_collector_mainnet".to_string(),
            },
            testnet_config: NetworkConfig {
                environment: NetworkEnvironment::Testnet,
                gateway_config: GatewayConfig {
                    axelar_gateway: "terra1axelar_testnet".to_string(),
                    noble_gateway: "terra1noble_testnet".to_string(),
                },
                registry_address: testnet_registry,
                oracle_address: "terra1oracle_testnet".to_string(),
                fee_collector_address: "terra1fee_collector_testnet".to_string(),
            },
            localnet_config: NetworkConfig {
                environment: NetworkEnvironment::LocalNet,
                gateway_config: GatewayConfig {
                    axelar_gateway: "terra1axelar_localnet".to_string(), 
                    noble_gateway: "terra1noble_localnet".to_string(),
                },
                registry_address: localnet_registry,
                oracle_address: "terra1oracle_localnet".to_string(),
                fee_collector_address: "terra1fee_collector_localnet".to_string(),
            },
        };
        
        GLOBAL_CONFIG.save(storage, &config)
    }
    
    /// Update the active network environment
    pub fn set_active_network(
        storage: &mut dyn Storage,
        network: NetworkEnvironment,
    ) -> StdResult<()> {
        GLOBAL_CONFIG.update(storage, |mut config| -> StdResult<_> {
            config.active_network = network;
            Ok(config)
        })?;
        Ok(())
    }
    
    /// Toggle fallback functionality
    pub fn set_fallback_enabled(
        storage: &mut dyn Storage,
        enabled: bool,
    ) -> StdResult<()> {
        GLOBAL_CONFIG.update(storage, |mut config| -> StdResult<_> {
            config.fallback_enabled = enabled;
            Ok(config)
        })?;
        Ok(())
    }
}
