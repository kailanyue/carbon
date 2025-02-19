use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf8c69e91e17587c8")]
pub struct Swap {
    pub amount: u64,
    pub other_amount_threshold: u64,
    pub sqrt_price_limit: u128,
    pub amount_specified_is_input: bool,
    pub a_to_b: bool,
}

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf8c69e91e17587c1")]
pub struct SwapInstructionAccounts {
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub token_authority: solana_sdk::pubkey::Pubkey,
    pub whirlpool: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_a: solana_sdk::pubkey::Pubkey,
    pub token_vault_a: solana_sdk::pubkey::Pubkey,
    pub token_owner_account_b: solana_sdk::pubkey::Pubkey,
    pub token_vault_b: solana_sdk::pubkey::Pubkey,
    pub tick_array0: solana_sdk::pubkey::Pubkey,
    pub tick_array1: solana_sdk::pubkey::Pubkey,
    pub tick_array2: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for Swap {
    type ArrangedAccounts = SwapInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [token_program, token_authority, whirlpool, token_owner_account_a, token_vault_a, token_owner_account_b, token_vault_b, tick_array0, tick_array1, tick_array2, oracle] =
            accounts
        else {
            return None;
        };

        Some(SwapInstructionAccounts {
            token_program: token_program.pubkey,
            token_authority: token_authority.pubkey,
            whirlpool: whirlpool.pubkey,
            token_owner_account_a: token_owner_account_a.pubkey,
            token_vault_a: token_vault_a.pubkey,
            token_owner_account_b: token_owner_account_b.pubkey,
            token_vault_b: token_vault_b.pubkey,
            tick_array0: tick_array0.pubkey,
            tick_array1: tick_array1.pubkey,
            tick_array2: tick_array2.pubkey,
            oracle: oracle.pubkey,
        })
    }
}
