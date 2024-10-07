use carbon_core::borsh;
use carbon_core::deserialize::{ArrangeAccounts, CarbonDeserialize};
use carbon_proc_macros::CarbonDeserialize;

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x07")]
pub struct AuthorizeNonceAccount(solana_sdk::pubkey::Pubkey);

pub struct AuthorizeNonceAccountAccounts {
    pub nonce_account: solana_sdk::pubkey::Pubkey,
    pub nonce_authority: solana_sdk::pubkey::Pubkey,
}

impl ArrangeAccounts for AuthorizeNonceAccount {
    type ArrangedAccounts = AuthorizeNonceAccountAccounts;

    fn arrange_accounts(
        &self,
        accounts: Vec<solana_sdk::pubkey::Pubkey>,
    ) -> Option<Self::ArrangedAccounts> {
        let nonce_account = accounts.get(0)?;
        let nonce_authority = accounts.get(1)?;

        Some(AuthorizeNonceAccountAccounts {
            nonce_account: *nonce_account,
            nonce_authority: *nonce_authority,
        })
    }
}