

use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;
use carbon_core::borsh;


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xdb8649dbb4075ece")]
pub struct CloseOrderBook{
}

pub struct CloseOrderBookInstructionAccounts {
    pub order_book: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for CloseOrderBook {
    type ArrangedAccounts = CloseOrderBookInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let order_book = accounts.get(0)?;
        let payer = accounts.get(1)?;

        Some(CloseOrderBookInstructionAccounts {
            order_book: order_book.pubkey,
            payer: payer.pubkey,
        })
    }
}