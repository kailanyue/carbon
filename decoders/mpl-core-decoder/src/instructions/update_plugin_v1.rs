
use super::super::types::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x06")]
pub struct UpdatePluginV1{
    pub update_plugin_v1_args: UpdatePluginV1Args,
}

pub struct UpdatePluginV1InstructionAccounts {
    pub asset: solana_sdk::pubkey::Pubkey,
    pub collection: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub log_wrapper: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdatePluginV1 {
    type ArrangedAccounts = UpdatePluginV1InstructionAccounts;

    fn arrange_accounts(accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let asset = accounts.get(0)?;
        let collection = accounts.get(1)?;
        let payer = accounts.get(2)?;
        let authority = accounts.get(3)?;
        let system_program = accounts.get(4)?;
        let log_wrapper = accounts.get(5)?;

        Some(UpdatePluginV1InstructionAccounts {
            asset: asset.pubkey,
            collection: collection.pubkey,
            payer: payer.pubkey,
            authority: authority.pubkey,
            system_program: system_program.pubkey,
            log_wrapper: log_wrapper.pubkey,
        })
    }
}