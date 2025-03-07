#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env};
    use equilibria_smart_contracts::config::{
        NetworkEnvironment, storage as config_storage
    };
    use equilibria_smart_contracts::network::NetworkManager;
    
    #[test]
    fn test_config_initialization() {
        let mut deps = mock_dependencies();
        
        // Initialize configuration
        config_storage::initialize_config(
            deps.as_mut().storage,
            "terra1registry_mainnet".to_string(),
            "terra1registry_testnet".to_string(),
            "terra1registry_localnet".to_string(),
        ).unwrap();
        
        // Load the saved configuration
        let config = config_storage::GLOBAL_CONFIG.load(deps.as_ref().storage).unwrap();
        
        // Verify default values
        assert_eq!(config.active_network, NetworkEnvironment::Mainnet);
        assert_eq!(config.fallback_enabled, true);
        assert_eq!(config.mainnet_config.registry_address, "terra1registry_mainnet");
        assert_eq!(config.testnet_config.registry_address, "terra1registry_testnet");
        assert_eq!(config.localnet_config.registry_address, "terra1registry_localnet");
    }
    
    #[test]
    fn test_fallback_behavior() {
        let mut deps = mock_dependencies();
        
        // Initialize configuration
        config_storage::initialize_config(
            deps.as_mut().storage,
            "terra1registry_mainnet".to_string(),
            "terra1registry_testnet".to_string(),
            "terra1registry_localnet".to_string(),
        ).unwrap();
        
        // Load the configuration
        let config = config_storage::GLOBAL_CONFIG.load(deps.as_ref().storage).unwrap();
        
        // Mainnet selected with fallback enabled - should use mainnet
        let active_config = config.get_active_config();
        assert_eq!(active_config.environment, NetworkEnvironment::Mainnet);
        
        // Simulate disabling fallback and changing to testnet
        let mut updated_config = config.clone();
        updated_config.fallback_enabled = false;
        updated_config.active_network = NetworkEnvironment::Testnet;
        config_storage::GLOBAL_CONFIG.save(deps.as_mut().storage, &updated_config).unwrap();
        
        // Now should use testnet config since fallback is disabled
        let config = config_storage::GLOBAL_CONFIG.load(deps.as_ref().storage).unwrap();
        let active_config = config.get_active_config();
        assert_eq!(active_config.environment, NetworkEnvironment::Testnet);
        
        // Re-enable fallback
        let mut updated_config = config.clone();
        updated_config.fallback_enabled = true;
        config_storage::GLOBAL_CONFIG.save(deps.as_mut().storage, &updated_config).unwrap();
        
        // Load again and verify we're using mainnet again due to fallback
        let config = config_storage::GLOBAL_CONFIG.load(deps.as_ref().storage).unwrap();
        let active_config = config.get_active_config();
        assert_eq!(active_config.environment, NetworkEnvironment::Mainnet);
    }
    
    #[test]
    fn test_network_detection() {
        // Create mock environment with mainnet chain id
        let mut env = mock_env();
        env.block.chain_id = "phoenix-1".to_string();
        
        // Detect network
        let network = NetworkManager::detect_network(&env);
        assert_eq!(network, NetworkEnvironment::Mainnet);
        
        // Test testnet detection
        env.block.chain_id = "pisco-1".to_string();
        let network = NetworkManager::detect_network(&env);
        assert_eq!(network, NetworkEnvironment::Testnet);
        
        // Test localnet detection
        env.block.chain_id = "localterra-1".to_string();
        let network = NetworkManager::detect_network(&env);
        assert_eq!(network, NetworkEnvironment::LocalNet);
    }
}
