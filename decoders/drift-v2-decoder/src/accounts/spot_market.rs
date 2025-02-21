use super::super::types::*;

use carbon_core::{borsh, CarbonDeserialize};

#[derive(CarbonDeserialize, Debug)]
#[carbon(discriminator = "0x64b1086ba8414127")]
pub struct SpotMarket {
    pub pubkey: solana_sdk::pubkey::Pubkey,
    pub oracle: solana_sdk::pubkey::Pubkey,
    pub mint: solana_sdk::pubkey::Pubkey,
    pub vault: solana_sdk::pubkey::Pubkey,
    pub name: [u8; 32],
    pub historical_oracle_data: HistoricalOracleData,
    pub historical_index_data: HistoricalIndexData,
    pub revenue_pool: PoolBalance,
    pub spot_fee_pool: PoolBalance,
    pub insurance_fund: InsuranceFund,
    pub total_spot_fee: u128,
    pub deposit_balance: u128,
    pub borrow_balance: u128,
    pub cumulative_deposit_interest: u128,
    pub cumulative_borrow_interest: u128,
    pub total_social_loss: u128,
    pub total_quote_social_loss: u128,
    pub withdraw_guard_threshold: u64,
    pub max_token_deposits: u64,
    pub deposit_token_twap: u64,
    pub borrow_token_twap: u64,
    pub utilization_twap: u64,
    pub last_interest_ts: u64,
    pub last_twap_ts: u64,
    pub expiry_ts: i64,
    pub order_step_size: u64,
    pub order_tick_size: u64,
    pub min_order_size: u64,
    pub max_position_size: u64,
    pub next_fill_record_id: u64,
    pub next_deposit_record_id: u64,
    pub initial_asset_weight: u32,
    pub maintenance_asset_weight: u32,
    pub initial_liability_weight: u32,
    pub maintenance_liability_weight: u32,
    pub imf_factor: u32,
    pub liquidator_fee: u32,
    pub if_liquidation_fee: u32,
    pub optimal_utilization: u32,
    pub optimal_borrow_rate: u32,
    pub max_borrow_rate: u32,
    pub decimals: u32,
    pub market_index: u16,
    pub orders_enabled: bool,
    pub oracle_source: OracleSource,
    pub status: MarketStatus,
    pub asset_tier: AssetTier,
    pub paused_operations: u8,
    pub if_paused_operations: u8,
    pub fee_adjustment: i16,
    pub max_token_borrows_fraction: u16,
    pub flash_loan_amount: u64,
    pub flash_loan_initial_token_amount: u64,
    pub total_swap_fee: u64,
    pub scale_initial_asset_weight_start: u64,
    pub min_borrow_rate: u8,
    pub fuel_boost_deposits: u8,
    pub fuel_boost_borrows: u8,
    pub fuel_boost_taker: u8,
    pub fuel_boost_maker: u8,
    pub fuel_boost_insurance: u8,
    pub token_program: u8,
    pub pool_id: u8,
    pub padding: [u8; 40],
}
