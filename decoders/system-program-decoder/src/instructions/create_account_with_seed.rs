use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x03")]
pub struct CreateAccountWithSeed {
    pub base: solana_sdk::pubkey::Pubkey,
    pub seed: String,
    pub lamports: u64,
    pub space: u64,
    pub owner: solana_sdk::pubkey::Pubkey,
}

pub struct CreateAccountWithSeedAccounts {
    pub funding_account: solana_sdk::pubkey::Pubkey,
    pub created_account: solana_sdk::pubkey::Pubkey,
    pub base_account: Option<solana_sdk::pubkey::Pubkey>,
}

impl ArrangeAccounts for CreateAccountWithSeed {
    type ArrangedAccounts = CreateAccountWithSeedAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let funding_account = accounts.get(0)?;
        let created_account = accounts.get(1)?;
        let base_account = if let Some(acc) = accounts.get(2).cloned() {
            Some(acc)
        } else {
            None
        };

        Some(CreateAccountWithSeedAccounts {
            funding_account: *funding_account,
            created_account: *created_account,
            base_account,
        })
    }
}