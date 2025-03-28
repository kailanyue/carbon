use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x04")]
pub struct Approve {
    pub amount: u64,
}

pub struct ApproveAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub delegate: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl carbon_core::deserialize::ArrangeAccounts for Approve {
    type ArrangedAccounts = ApproveAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, delegate, owner, remaining_accounts @ ..] = accounts else {
            return None;
        };

        Some(ApproveAccounts {
            source: source.pubkey,
            delegate: delegate.pubkey,
            owner: owner.pubkey,
            remaining_accounts: remaining_accounts.to_vec(),
        })
    }
}
