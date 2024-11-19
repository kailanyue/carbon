

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x6cd1044815167685")]
pub struct DepositObligationCollateral{
    pub collateral_amount: u64,
}

pub struct DepositObligationCollateralInstructionAccounts {
    pub owner: solana_sdk::pubkey::Pubkey,
    pub obligation: solana_sdk::pubkey::Pubkey,
    pub lending_market: solana_sdk::pubkey::Pubkey,
    pub deposit_reserve: solana_sdk::pubkey::Pubkey,
    pub reserve_destination_collateral: solana_sdk::pubkey::Pubkey,
    pub user_source_collateral: solana_sdk::pubkey::Pubkey,
    pub token_program: solana_sdk::pubkey::Pubkey,
    pub instruction_sysvar_account: solana_sdk::pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for DepositObligationCollateral {
    type ArrangedAccounts = DepositObligationCollateralInstructionAccounts;

    fn arrange_accounts(&self, accounts: Vec<solana_sdk::instruction::AccountMeta>) -> Option<Self::ArrangedAccounts> {
        let owner = accounts.get(0)?;
        let obligation = accounts.get(1)?;
        let lending_market = accounts.get(2)?;
        let deposit_reserve = accounts.get(3)?;
        let reserve_destination_collateral = accounts.get(4)?;
        let user_source_collateral = accounts.get(5)?;
        let token_program = accounts.get(6)?;
        let instruction_sysvar_account = accounts.get(7)?;

        Some(DepositObligationCollateralInstructionAccounts {
            owner: owner.pubkey,
            obligation: obligation.pubkey,
            lending_market: lending_market.pubkey,
            deposit_reserve: deposit_reserve.pubkey,
            reserve_destination_collateral: reserve_destination_collateral.pubkey,
            user_source_collateral: user_source_collateral.pubkey,
            token_program: token_program.pubkey,
            instruction_sysvar_account: instruction_sysvar_account.pubkey,
        })
    }
}