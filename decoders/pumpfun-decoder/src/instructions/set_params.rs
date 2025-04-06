use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1beab2349302bb8d")]
pub struct SetParams {
    pub initial_virtual_token_reserves: u64,
    pub initial_virtual_sol_reserves: u64,
    pub initial_real_token_reserves: u64,
    pub token_total_supply: u64,
    pub fee_basis_points: u64,
    pub withdraw_authority: solana_pubkey::Pubkey,
    pub enable_migrate: bool,
    pub pool_migration_fee: u64,
    pub creator_fee: u64,
}

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x1beab2349302bb81")]
pub struct SetParamsInstructionAccounts {
    pub global: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub event_authority: solana_pubkey::Pubkey,
    pub program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for SetParams {
    type ArrangedAccounts = SetParamsInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [global, authority, event_authority, program, _remaining @ ..] = accounts else {
            return None;
        };

        Some(SetParamsInstructionAccounts {
            global: global.pubkey,
            authority: authority.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
