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
