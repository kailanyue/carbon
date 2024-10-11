 
use carbon_core::deserialize::CarbonDeserialize; 
use carbon_proc_macros::CarbonDeserialize; 
use carbon_core::borsh;
use super::super::types::*;

#[derive(CarbonDeserialize, Debug)] 
 

#[carbon(discriminator = "0x525d5a7f2865919a")] 
pub struct Dca { 
        pub user: solana_sdk::pubkey::Pubkey, 
        pub input_mint: solana_sdk::pubkey::Pubkey, 
        pub output_mint: solana_sdk::pubkey::Pubkey, 
        pub idx: u64, 
        pub next_cycle_at: i64, 
        pub in_deposited: u64, 
        pub in_withdrawn: u64, 
        pub out_withdrawn: u64, 
        pub in_used: u64, 
        pub out_received: u64, 
        pub in_amount_per_cycle: u64, 
        pub cycle_frequency: i64, 
        pub next_cycle_amount_left: u64, 
        pub in_account: solana_sdk::pubkey::Pubkey, 
        pub out_account: solana_sdk::pubkey::Pubkey, 
        pub min_out_amount: u64, 
        pub max_out_amount: u64, 
        pub keeper_in_balance_before_borrow: u64, 
        pub dca_out_balance_before_swap: u64, 
        pub created_at: i64, 
        pub bump: u8, 
}