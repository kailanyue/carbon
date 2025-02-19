use carbon_core::{borsh, CarbonDeserialize};
#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x0c")]
pub struct TransferChecked {
    pub amount: u64,
    pub decimals: u8,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone)]
pub struct TransferCheckedAccounts {
    pub source: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub destination: solana_sdk::pubkey::Pubkey,
    pub authority: solana_sdk::pubkey::Pubkey,
    pub remaining_accounts: Vec<solana_sdk::instruction::AccountMeta>,
}

impl carbon_core::deserialize::ArrangeAccounts for TransferChecked {
    type ArrangedAccounts = TransferCheckedAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [source, mint, destination, authority] = accounts else {
            return None;
        };

        Some(TransferCheckedAccounts {
            source: source.pubkey,
            mint: mint.pubkey,
            destination: destination.pubkey,
            authority: authority.pubkey,
            remaining_accounts: accounts.get(4..).unwrap_or_default().to_vec(),
        })
    }
}
