use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, Addr, Decimal, Uint128};

// Import the necessary modules from each contract
use eqa_token::contract::{execute_mint, execute_redeem};
use eqa_token::lib::{ExecuteMsg as EqaExecuteMsg, InstantiateMsg as EqaInstantiateMsg};

use collateral_manager::contract::execute_update_collateral;
use collateral_manager::lib::{ExecuteMsg as CollateralExecuteMsg};

use liquidation_engine::contract::execute_check_liquidation;

#[test]
fn test_minting_and_redemption() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let admin_info = mock_info("admin", &[]);
    let user_info = mock_info("user", &[]);
    
    // Initialize EQA token
    let eqa_init_msg = EqaInstantiateMsg {
        name: "Equilibria".to_string(),
        symbol: "EQA".to_string(),
        decimals: 6,
    };
    eqa_token::lib::instantiate(deps.as_mut(), env.clone(), admin_info.clone(), eqa_init_msg).unwrap();
    
    // Mint some EQA tokens (100 EQA)
    let mint_msg = EqaExecuteMsg::Mint {
        amount: Uint128::new(100_000_000), // 100 tokens with 6 decimal places
        market_price: Decimal::one(),      // 1.0 price (at peg)
    };
    let res = eqa_token::lib::execute(deps.as_mut(), env.clone(), user_info.clone(), mint_msg).unwrap();
    
    // Check that minting emits the correct attributes
    assert_eq!(res.attributes[0].key, "action");
    assert_eq!(res.attributes[0].value, "mint");
    
    // Check user balance after minting
    let query_res = eqa_token::lib::query(
        deps.as_ref(), 
        env.clone(), 
        eqa_token::lib::QueryMsg::Balance { 
            address: "user".to_string() 
        }
    ).unwrap();
    let balance: eqa_token::lib::BalanceResponse = serde_json::from_slice(&query_res).unwrap();
    
    // Expected balance should account for the 1% fee
    let expected_balance = Uint128::new(99_000_000); // 100M - 1% fee
    assert_eq!(balance.balance, expected_balance);
    
    // Now test redemption
    let redeem_msg = EqaExecuteMsg::Redeem {
        amount: Uint128::new(50_000_000), // Redeem 50 tokens
        market_price: Decimal::one(),      // At peg price
    };
    let res = eqa_token::lib::execute(deps.as_mut(), env.clone(), user_info.clone(), redeem_msg).unwrap();
    
    // Check that redemption emits correct attributes
    assert_eq!(res.attributes[0].key, "action");
    assert_eq!(res.attributes[0].value, "redeem");
    
    // Check balance after redemption
    let query_res = eqa_token::lib::query(
        deps.as_ref(), 
        env.clone(), 
        eqa_token::lib::QueryMsg::Balance { 
            address: "user".to_string() 
        }
    ).unwrap();
    let balance: eqa_token::lib::BalanceResponse = serde_json::from_slice(&query_res).unwrap();
    
    // Expected balance: 99M - 50M = 49M
    let expected_balance = Uint128::new(49_000_000);
    assert_eq!(balance.balance, expected_balance);
}

#[test]
fn test_collateral_and_liquidation() {
    let mut deps = mock_dependencies();
    let env = mock_env();
    let admin_info = mock_info("admin", &[]);
    
    // Initialize collateral manager
    collateral_manager::lib::instantiate(
        deps.as_mut(), 
        env.clone(), 
        admin_info.clone(), 
        collateral_manager::lib::InstantiateMsg {}
    ).unwrap();
    
    // Update collateral (100,000 USDC)
    let collateral_msg = CollateralExecuteMsg::UpdateCollateral { 
        usdc_axelar: Uint128::new(60_000_000_000), // 60,000 USDC with 6 decimal places
        usdc_noble: Uint128::new(40_000_000_000),  // 40,000 USDC with 6 decimal places
    };
    let res = collateral_manager::lib::execute(
        deps.as_mut(), 
        env.clone(), 
        admin_info.clone(), 
        collateral_msg
    ).unwrap();
    
    assert_eq!(res.attributes[0].key, "action");
    assert_eq!(res.attributes[0].value, "update_collateral");
    
    // Check collateral info
    let query_res = collateral_manager::lib::query(
        deps.as_ref(), 
        env.clone(), 
        collateral_manager::lib::QueryMsg::GetCollateralInfo {}
    ).unwrap();
    let collateral: collateral_manager::lib::CollateralResponse = serde_json::from_slice(&query_res).unwrap();
    
    assert_eq!(collateral.usdc_axelar, Uint128::new(60_000_000_000));
    assert_eq!(collateral.usdc_noble, Uint128::new(40_000_000_000));
    assert_eq!(collateral.total_locked, Uint128::new(100_000_000_000));
    
    // Now test liquidation engine
    // First initialize liquidation engine with 110% threshold
    liquidation_engine::lib::instantiate(
        deps.as_mut(),
        env.clone(),
        admin_info.clone(),
        liquidation_engine::lib::InstantiateMsg { threshold_ratio: 110 }
    ).unwrap();
    
    // Test solvency case (supply of 90,000 EQA, which should be solvent)
    let res = liquidation_engine::contract::execute_check_liquidation(
        deps.as_mut(),
        env.clone(),
        admin_info.clone(),
        Uint128::new(90_000_000_000), // 90,000 EQA
        Uint128::new(1_000_000)       // $1.00 price
    );
    
    // Should be solvent - no error
    assert!(res.is_ok());
    
    // Test insolvency case (supply of 100,000 EQA, which is at the edge)
    let res = liquidation_engine::contract::execute_check_liquidation(
        deps.as_mut(),
        env.clone(),
        admin_info.clone(),
        Uint128::new(95_000_000_000), // 95,000 EQA (>90,909 which is the max at 110% collateral ratio)
        Uint128::new(1_000_000)       // $1.00 price
    );
    
    // Should be insolvent - should return an error
    assert!(res.is_err());
}
