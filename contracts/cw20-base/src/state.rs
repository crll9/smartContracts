use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128, Timestamp};
use cw_storage_plus::{Item, Map};

use cw20::{AllowanceResponse, Logo, MarketingInfoResponse};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TokenInfo {
    pub name: String,
    pub symbol: String,
    pub decimals: u8,
    pub total_supply: Uint128,
    pub mint: Option<MinterData>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct MinterData {
    pub minter: Addr,
    /// cap is how many more tokens can be issued by the minter
    pub cap: Option<Uint128>,
}

impl TokenInfo {
    pub fn get_cap(&self) -> Option<Uint128> {
        self.mint.as_ref().and_then(|v| v.cap)
    }
}

/// This is used for saving various vesting details
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct VestingDetails {
    /// The system timestamp to be used as starting point of vesting schedule
    pub vesting_start_timestamp: Timestamp,

    /// Initial seed amount. The seed quantity of tokens
    pub initial_vesting_count: Uint128,
    
    /// amount claimed fom initial seed
    pub initial_vesting_consumed: Uint128,
    
    /// The vesting periodicity(hourly/daily/weekly/monthly) expressed in seconds
    pub vesting_periodicity: u64,
    
    /// vesting count for each period
    pub vesting_count_per_period: Uint128,
    
    /// Total amount to be vested over period of time 
    /// This also includes the seed tokens
    pub total_vesting_token_count: Uint128,
    
    /// Total number of tokens transferred till date
    /// This also includes the seed tokens 
    pub total_claimed_tokens_till_now: Uint128,
    
    /// Timestamp for the latest claimed/transferred transaction.
    /// This will be used to calculate the next vesting count
    pub last_claimed_timestamp: Option<Timestamp>,
    
    /// Number of tokens available for claiming
    pub tokens_available_to_claim: Uint128,

    /// Timestamp for the latest claimed/transferred transaction.
    /// This will be used to calculate the next vesting count
    pub last_vesting_timestamp: Option<Timestamp>,
    
    /// Cliff time period expressed in months
    pub cliff_period: u64,

    /// This contains addredd of parent category. It will be none for all main categories. 
    pub category_address: Option<String>,
}


pub const TOKEN_INFO: Item<TokenInfo> = Item::new("token_info");
pub const MARKETING_INFO: Item<MarketingInfoResponse> = Item::new("marketing_info");
pub const LOGO: Item<Logo> = Item::new("logo");
pub const BALANCES: Map<&Addr, Uint128> = Map::new("balance");
pub const ALLOWANCES: Map<(&Addr, &Addr), AllowanceResponse> = Map::new("allowance");
pub const VESTING_DETAILS: Map<&Addr, VestingDetails> = Map::new("vesting_details");
