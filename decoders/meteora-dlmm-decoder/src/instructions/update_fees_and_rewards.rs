use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9ae6fa0decd14bdf")]
pub struct UpdateFeesAndRewards {}

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x9ae6fa0decd14be0")]
pub struct UpdateFeesAndRewardsInstructionAccounts {
    pub position: solana_sdk::pubkey::Pubkey,
    pub lb_pair: solana_sdk::pubkey::Pubkey,
    pub bin_array_lower: solana_sdk::pubkey::Pubkey,
    pub bin_array_upper: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for UpdateFeesAndRewards {
    type ArrangedAccounts = UpdateFeesAndRewardsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [position, lb_pair, bin_array_lower, bin_array_upper, owner] = accounts else {
            return None;
        };

        Some(UpdateFeesAndRewardsInstructionAccounts {
            position: position.pubkey,
            lb_pair: lb_pair.pubkey,
            bin_array_lower: bin_array_lower.pubkey,
            bin_array_upper: bin_array_upper.pubkey,
            owner: owner.pubkey,
        })
    }
}
