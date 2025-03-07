// This module contains mock implementations for testing

use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage},
    OwnedDeps, Decimal, to_json_binary, Empty, Querier,
};

use crate::oracle::OracleResponse;

// Create mock dependencies with a custom querier
pub fn mock_dependencies_with_custom_querier(
    price: Option<Decimal>,
) -> OwnedDeps<MockStorage, MockApi, MockCustomQuerier, Empty> {
    let oracle_price = price.unwrap_or_else(Decimal::one);
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: MockCustomQuerier::new(oracle_price),
        custom_query_type: std::marker::PhantomData,
    }
}

pub struct MockCustomQuerier {
    base: MockQuerier,
    oracle_price: Decimal,
}

impl MockCustomQuerier {
    pub fn new(oracle_price: Decimal) -> Self {
        Self {
            base: MockQuerier::default(),
            oracle_price,
        }
    }
}

// Implement the Querier trait for our custom querier
impl Querier for MockCustomQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> cosmwasm_std::QuerierResult {
        let request: cosmwasm_std::QueryRequest<Empty> = cosmwasm_std::from_json(bin_request).unwrap();
        
        match request {
            cosmwasm_std::QueryRequest::Wasm(cosmwasm_std::WasmQuery::Smart { contract_addr, msg }) => {
                // Mock oracle queries
                if contract_addr == "terra1oracle_mainnet" || 
                   contract_addr == "terra1oracle_testnet" ||
                   contract_addr == "terra1oracle_localnet" {
                    
                    // Parse query message
                    let query_msg: crate::oracle::OracleQueryMsg = cosmwasm_std::from_json(msg.as_slice()).unwrap();
                    
                    match query_msg {
                        crate::oracle::OracleQueryMsg::GetPrice { denom: _ } => {
                            // Return mock price response
                            let response = OracleResponse {
                                price: self.oracle_price,
                                last_updated: 123456789,
                                source: "mock_oracle".to_string(),
                            };
                            cosmwasm_std::SystemResult::Ok(
                                cosmwasm_std::ContractResult::Ok(to_json_binary(&response).unwrap())
                            )
                        },
                        crate::oracle::OracleQueryMsg::GetExchangeRate { .. } => {
                            // Return error for this query type - not implemented in mock
                            cosmwasm_std::SystemResult::Err(
                                cosmwasm_std::SystemError::NoSuchContract { 
                                    addr: "GetExchangeRate not implemented in mock".to_string() 
                                }
                            )
                        },
                        crate::oracle::OracleQueryMsg::GetPrices { .. } => {
                            // Return error for this query type - not implemented in mock
                            cosmwasm_std::SystemResult::Err(
                                cosmwasm_std::SystemError::NoSuchContract { 
                                    addr: "GetPrices not implemented in mock".to_string() 
                                }
                            )
                        },
                    }
                } else {
                    // Pass other queries to base querier
                    self.base.raw_query(bin_request)
                }
            },
            // Handle other query types
            _ => self.base.raw_query(bin_request),
        }
    }
}

// Mock implementations for external dependencies
#[cfg(test)]
pub mod terra_cosmwasm {
    use cosmwasm_std::Deps;
    use cosmwasm_std::StdResult;
    
    pub struct ExchangeRateResponse {
        pub exchange_rate: DecimalWrapper,
    }
    
    pub struct DecimalWrapper {
        value: u128,
    }
    
    impl DecimalWrapper {
        pub fn numerator(&self) -> u128 {
            self.value
        }
        
        pub fn denominator(&self) -> u128 {
            1u128
        }
    }
    
    pub struct TerraQuerier<'a> {
        _deps: &'a Deps<'a>,
    }
    
    impl<'a> TerraQuerier<'a> {
        pub fn new(deps: &'a Deps) -> Self {
            Self {
                _deps: deps,
            }
        }
        
        pub fn query_exchange_rate(&self, _base: String, _quote: String) -> StdResult<ExchangeRateResponse> {
            // In tests, always return a 1:1 exchange rate
            Ok(ExchangeRateResponse {
                exchange_rate: DecimalWrapper {
                    value: 1,
                }
            })
        }
    }
}
