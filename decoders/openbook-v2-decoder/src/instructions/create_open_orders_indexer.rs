use carbon_core::{borsh, CarbonDeserialize};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x404099ffd947f985")]
pub struct CreateOpenOrdersIndexer {}

pub struct CreateOpenOrdersIndexerInstructionAccounts {
    pub payer: solana_sdk::pubkey::Pubkey,
    pub owner: solana_sdk::pubkey::Pubkey,
    pub open_orders_indexer: solana_sdk::pubkey::Pubkey,
    pub system_program: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateOpenOrdersIndexer {
    type ArrangedAccounts = CreateOpenOrdersIndexerInstructionAccounts;

    fn arrange_accounts(
        accounts: Vec<solana_sdk::instruction::AccountMeta>,
    ) -> Option<Self::ArrangedAccounts> {
        let payer = accounts.get(0)?;
        let owner = accounts.get(1)?;
        let open_orders_indexer = accounts.get(2)?;
        let system_program = accounts.get(3)?;

        Some(CreateOpenOrdersIndexerInstructionAccounts {
            payer: payer.pubkey,
            owner: owner.pubkey,
            open_orders_indexer: open_orders_indexer.pubkey,
            system_program: system_program.pubkey,
        })
    }
}
