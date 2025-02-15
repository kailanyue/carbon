

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum EscrowAuthority {
    TokenOwner,
    Creator
                (
                    solana_sdk::pubkey::Pubkey,
                )
    ,
}


