
use super::*;
use carbon_core::deserialize::CarbonDeserialize;
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct AddLiquidity {
    pub token_amount_in: u64,
    pub min_lp_amount_out: u64,
    pub token_amount_pre_swap: Option<u64>,
}