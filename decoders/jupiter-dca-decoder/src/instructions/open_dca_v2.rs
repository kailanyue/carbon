use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x8e772b6da2340bb1")]
pub struct OpenDcaV2 {
    pub application_idx: u64,
    pub in_amount: u64,
    pub in_amount_per_cycle: u64,
    pub cycle_frequency: i64,
    pub min_out_amount: Option<u64>,
    pub max_out_amount: Option<u64>,
    pub start_at: Option<i64>,
}

pub struct OpenDcaV2InstructionAccounts {
    pub dca: solana_sdk::pubkey::Pubkey,
    pub user: solana_sdk::pubkey::Pubkey,
    pub payer: solana_sdk::pubkey::Pubkey,
    pub input_mint: solana_sdk::pubkey::Pubkey,
    pub output_mint: solana_sdk::pubkey::Pubkey,
    pub user_ata: solana_sdk::pubkey::Pubkey,
    pub in_ata: solana_sdk::pubkey::Pubkey,
    pub out_ata: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub associated_token_program: solana_sdk::pubkey::Pubkey,
    pub event_authority: solana_sdk::pubkey::Pubkey,
    pub program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for OpenDcaV2 {
    type ArrangedAccounts = OpenDcaV2InstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_sdk::instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let [dca, user, payer, input_mint, output_mint, user_ata, in_ata, out_ata, system_program, token_program, associated_token_program, event_authority, program] =
            accounts
        else {
            return None;
        };

        Some(OpenDcaV2InstructionAccounts {
            dca: dca.pubkey,
            user: user.pubkey,
            payer: payer.pubkey,
            input_mint: input_mint.pubkey,
            output_mint: output_mint.pubkey,
            user_ata: user_ata.pubkey,
            in_ata: in_ata.pubkey,
            out_ata: out_ata.pubkey,
            system_program: system_program.pubkey,
            token_program: token_program.pubkey,
            associated_token_program: associated_token_program.pubkey,
            event_authority: event_authority.pubkey,
            program: program.pubkey,
        })
    }
}
