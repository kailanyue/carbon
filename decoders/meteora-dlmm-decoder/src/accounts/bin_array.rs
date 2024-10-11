 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x5c8e5cdc059446b5")] 
pub struct BinArray { 
        pub index: i64, 
        pub version: u8, 
        pub padding: [u8; 7], 
        pub lb_pair: solana_sdk::pubkey::Pubkey, 
        pub bins: [Bin; 70], 
}