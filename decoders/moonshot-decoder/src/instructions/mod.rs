use crate::PROGRAM_ID;

use super::MoonshotDecoder;
pub mod buy;
pub mod config_init;
pub mod config_update;
pub mod migrate_funds;
pub mod migration_event;
pub mod sell;
pub mod token_mint;
pub mod trade_event;

#[derive(
    carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone,
)]
pub enum MoonshotInstruction {
    TokenMint(token_mint::TokenMint),
    Buy(buy::Buy),
    Sell(sell::Sell),
    MigrateFunds(migrate_funds::MigrateFunds),
    ConfigInit(config_init::ConfigInit),
    ConfigUpdate(config_update::ConfigUpdate),
    TradeEvent(trade_event::TradeEvent),
    MigrationEvent(migration_event::MigrationEvent),
}

impl carbon_core::instruction::InstructionDecoder<'_> for MoonshotDecoder {
    type InstructionType = MoonshotInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        if !instruction.program_id.eq(&PROGRAM_ID) {
            return None;
        }

        carbon_core::try_decode_instructions!(instruction,
            MoonshotInstruction::TokenMint => token_mint::TokenMint,
            MoonshotInstruction::Buy => buy::Buy,
            MoonshotInstruction::Sell => sell::Sell,
            MoonshotInstruction::MigrateFunds => migrate_funds::MigrateFunds,
            MoonshotInstruction::ConfigInit => config_init::ConfigInit,
            MoonshotInstruction::ConfigUpdate => config_update::ConfigUpdate,
            MoonshotInstruction::TradeEvent => trade_event::TradeEvent,
            MoonshotInstruction::MigrationEvent => migration_event::MigrationEvent,
        )
    }
}

#[cfg(test)]
mod tests {
    use carbon_core::{
        deserialize::{ArrangeAccounts, PrefixString},
        instruction::InstructionDecoder,
    };
    use solana_sdk::{instruction::AccountMeta, pubkey};

    use crate::types::{TokenMintParams, TradeParams};

    use super::*;

    #[test]
    fn test_decode_token_mint() {
        // Arrange
        let expected_ix = MoonshotInstruction::TokenMint(token_mint::TokenMint {
            mint_params: TokenMintParams {
                name: PrefixString("Gamestop".to_string()),
                symbol: PrefixString("$GME".to_string()),
                uri: PrefixString(
                    "https://cdn.dexscreener.com/cms/tokens/metadata/UgnNvayhAu8K97aoT3B8"
                        .to_string(),
                ),
                decimals: 9,
                collateral_currency: 0,
                amount: 1000000000000000000,
                curve_type: 1,
                migration_target: 0,
                ..TokenMintParams::default()
            },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("AiG8HZSRociZpmHGmFnHQ3eQvFxVq4HUwxXcZKZJaEr1"),
                true,
            ),
            AccountMeta::new_readonly(
                pubkey!("Cb8Fnhp95f9dLxB3sYkNCbN3Mjxuc3v2uQZ7uVeqvNGB"),
                true,
            ),
            AccountMeta::new(
                pubkey!("ALPoEBuBD9GMWRQrMmWd81By5GCH8X9i9HiPWBf68n8C"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HkzfucJHWykNPSpKpduLVg2j45zafYhA1G51xPLumoon"),
                true,
            ),
            AccountMeta::new(
                pubkey!("Do7VSKZ6yz1eJMg5N8GpZ5Me12jfXVTTaWTi6tgo3vFr"),
                false,
            ),
            AccountMeta::new(
                pubkey!("6oGTJKhWPZQ1ixiguEDvbHQJLizqUmbyL9isRvNUEGR"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("36Eru7v11oU5Pfrojyn5oY3nETA1a1iqsw2WUu6afkM9"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
        ];
        let expected_arranged_accounts = token_mint::TokenMintInstructionAccounts {
            sender: pubkey!("AiG8HZSRociZpmHGmFnHQ3eQvFxVq4HUwxXcZKZJaEr1"),
            backend_authority: pubkey!("Cb8Fnhp95f9dLxB3sYkNCbN3Mjxuc3v2uQZ7uVeqvNGB"),
            curve_account: pubkey!("ALPoEBuBD9GMWRQrMmWd81By5GCH8X9i9HiPWBf68n8C"),
            mint: pubkey!("HkzfucJHWykNPSpKpduLVg2j45zafYhA1G51xPLumoon"),
            mint_metadata: pubkey!("Do7VSKZ6yz1eJMg5N8GpZ5Me12jfXVTTaWTi6tgo3vFr"),
            curve_token_account: pubkey!("6oGTJKhWPZQ1ixiguEDvbHQJLizqUmbyL9isRvNUEGR"),
            config_account: pubkey!("36Eru7v11oU5Pfrojyn5oY3nETA1a1iqsw2WUu6afkM9"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            mpl_token_metadata: pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s"),
            system_program: pubkey!("11111111111111111111111111111111"),
        };

        // Act
        let decoder = MoonshotDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/token_mint_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            token_mint::TokenMint::arrange_accounts(&instruction.accounts)
                .expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_buy() {
        // Arrange
        let expected_ix = MoonshotInstruction::Buy(buy::Buy {
            data: TradeParams {
                token_amount: 5430576418647,
                collateral_amount: 1640000,
                fixed_side: 1,
                slippage_bps: 9999,
            },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("Ezug1uk7oTEULvBcXCngdZuJDmZ8Ed2TKY4oov4GmLLm"),
                true,
            ),
            AccountMeta::new(
                pubkey!("6FqNPPA4W1nuvL1BHGhusSHjdNa4qJBoXyRKggAh9pb9"),
                false,
            ),
            AccountMeta::new(
                pubkey!("4CYhuDhT4c9ATZpJceoQG8Du4vCjf5ZKvxsyXpJoVub4"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5Zg9kJdzYFKwS4hLzF1QvvNBYyUNpn9YWxYp6HVMknJt"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3udvfL24waJcLhskRAsStNMoNUvtyXdxrWQz4hgi953N"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5K5RtTWzzLp4P8Npi84ocf7F1vBsAu29N1irG4iiUnzt"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("3cBFsM1wosTJi9yun6kcHhYHyJcut1MNQY28zjC4moon"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("36Eru7v11oU5Pfrojyn5oY3nETA1a1iqsw2WUu6afkM9"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
        ];
        let expected_arranged_accounts = buy::BuyInstructionAccounts {
            sender: pubkey!("Ezug1uk7oTEULvBcXCngdZuJDmZ8Ed2TKY4oov4GmLLm"),
            sender_token_account: pubkey!("6FqNPPA4W1nuvL1BHGhusSHjdNa4qJBoXyRKggAh9pb9"),
            curve_account: pubkey!("4CYhuDhT4c9ATZpJceoQG8Du4vCjf5ZKvxsyXpJoVub4"),
            curve_token_account: pubkey!("5Zg9kJdzYFKwS4hLzF1QvvNBYyUNpn9YWxYp6HVMknJt"),
            dex_fee: pubkey!("3udvfL24waJcLhskRAsStNMoNUvtyXdxrWQz4hgi953N"),
            helio_fee: pubkey!("5K5RtTWzzLp4P8Npi84ocf7F1vBsAu29N1irG4iiUnzt"),
            mint: pubkey!("3cBFsM1wosTJi9yun6kcHhYHyJcut1MNQY28zjC4moon"),
            config_account: pubkey!("36Eru7v11oU5Pfrojyn5oY3nETA1a1iqsw2WUu6afkM9"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            system_program: pubkey!("11111111111111111111111111111111"),
        };

        // Act
        let decoder = MoonshotDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/buy_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            buy::Buy::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_sell() {
        // Arrange
        let expected_ix = MoonshotInstruction::Sell(sell::Sell {
            data: TradeParams {
                token_amount: 157227000000000,
                collateral_amount: 20990579,
                fixed_side: 0,
                slippage_bps: 100,
            },
        });
        let expected_accounts = vec![
            AccountMeta::new(
                pubkey!("93fdoNBQF6t7aBPuPv3SGGpXyWmJVfvWPpPsBXrGqEK7"),
                true,
            ),
            AccountMeta::new(
                pubkey!("H4QJQ3mm865pMW7Ufvq6BiSXn2P8xUCv2xFd1sWYpmmK"),
                false,
            ),
            AccountMeta::new(
                pubkey!("DnTTm5JdDoZS9pY5JxxJJ9LUQx5L3MmcR5DdvHyEDruQ"),
                false,
            ),
            AccountMeta::new(
                pubkey!("FNkJw68x21iyHrbA7yyUYyzFMmtdsNzxHWy7WwnaorEd"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3udvfL24waJcLhskRAsStNMoNUvtyXdxrWQz4hgi953N"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5K5RtTWzzLp4P8Npi84ocf7F1vBsAu29N1irG4iiUnzt"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("3hrY3mte6rpea8UDSm4Be6D1sUJyLyLpGxFfRBvVmoon"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("36Eru7v11oU5Pfrojyn5oY3nETA1a1iqsw2WUu6afkM9"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
        ];
        let expected_arranged_accounts = sell::SellInstructionAccounts {
            sender: pubkey!("93fdoNBQF6t7aBPuPv3SGGpXyWmJVfvWPpPsBXrGqEK7"),
            sender_token_account: pubkey!("H4QJQ3mm865pMW7Ufvq6BiSXn2P8xUCv2xFd1sWYpmmK"),
            curve_account: pubkey!("DnTTm5JdDoZS9pY5JxxJJ9LUQx5L3MmcR5DdvHyEDruQ"),
            curve_token_account: pubkey!("FNkJw68x21iyHrbA7yyUYyzFMmtdsNzxHWy7WwnaorEd"),
            dex_fee: pubkey!("3udvfL24waJcLhskRAsStNMoNUvtyXdxrWQz4hgi953N"),
            helio_fee: pubkey!("5K5RtTWzzLp4P8Npi84ocf7F1vBsAu29N1irG4iiUnzt"),
            mint: pubkey!("3hrY3mte6rpea8UDSm4Be6D1sUJyLyLpGxFfRBvVmoon"),
            config_account: pubkey!("36Eru7v11oU5Pfrojyn5oY3nETA1a1iqsw2WUu6afkM9"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
            system_program: pubkey!("11111111111111111111111111111111"),
        };

        // Act
        let decoder = MoonshotDecoder;
        let instruction = carbon_test_utils::read_instruction("tests/fixtures/sell_ix.json")
            .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            sell::Sell::arrange_accounts(&instruction.accounts).expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }

    #[test]
    fn test_decode_migrate_funds() {
        // Arrange
        let expected_ix = MoonshotInstruction::MigrateFunds(migrate_funds::MigrateFunds {});
        let expected_accounts = vec![
            AccountMeta::new_readonly(
                pubkey!("Cb8Fnhp95f9dLxB3sYkNCbN3Mjxuc3v2uQZ7uVeqvNGB"),
                true,
            ),
            AccountMeta::new(
                pubkey!("CGsqR7CTqTwbmAUTPnfg9Bj9GLJgkrUD9rhjh3vHEYvh"),
                true,
            ),
            AccountMeta::new(
                pubkey!("AwxvCNVa16VfZZdxtSZ2DJ8VDQ17J27FYMMJc9KoWkZt"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5JKuvPWQuYXAvvkCs7cvBFgmUdhPbaTW3ii7zGPknnHW"),
                false,
            ),
            AccountMeta::new(
                pubkey!("HzxySmjxfbV8nthWw7qogXZ9qH4LFUxSZVpU91n48xFf"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5JZUMLtHBzG4PaQ6UAhVWnh2pZ1BMMKRWWpCbMxxe148"),
                false,
            ),
            AccountMeta::new(
                pubkey!("3udvfL24waJcLhskRAsStNMoNUvtyXdxrWQz4hgi953N"),
                false,
            ),
            AccountMeta::new(
                pubkey!("5K5RtTWzzLp4P8Npi84ocf7F1vBsAu29N1irG4iiUnzt"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("36Eru7v11oU5Pfrojyn5oY3nETA1a1iqsw2WUu6afkM9"),
                false,
            ),
            AccountMeta::new_readonly(pubkey!("11111111111111111111111111111111"), false),
            AccountMeta::new_readonly(
                pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
                false,
            ),
            AccountMeta::new_readonly(
                pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
                false,
            ),
        ];
        let expected_arranged_accounts = migrate_funds::MigrateFundsInstructionAccounts {
            backend_authority: pubkey!("Cb8Fnhp95f9dLxB3sYkNCbN3Mjxuc3v2uQZ7uVeqvNGB"),
            migration_authority: pubkey!("CGsqR7CTqTwbmAUTPnfg9Bj9GLJgkrUD9rhjh3vHEYvh"),
            curve_account: pubkey!("AwxvCNVa16VfZZdxtSZ2DJ8VDQ17J27FYMMJc9KoWkZt"),
            curve_token_account: pubkey!("5JKuvPWQuYXAvvkCs7cvBFgmUdhPbaTW3ii7zGPknnHW"),
            migration_authority_token_account: pubkey!(
                "HzxySmjxfbV8nthWw7qogXZ9qH4LFUxSZVpU91n48xFf"
            ),
            mint: pubkey!("5JZUMLtHBzG4PaQ6UAhVWnh2pZ1BMMKRWWpCbMxxe148"),
            dex_fee_account: pubkey!("3udvfL24waJcLhskRAsStNMoNUvtyXdxrWQz4hgi953N"),
            helio_fee_account: pubkey!("5K5RtTWzzLp4P8Npi84ocf7F1vBsAu29N1irG4iiUnzt"),
            config_account: pubkey!("36Eru7v11oU5Pfrojyn5oY3nETA1a1iqsw2WUu6afkM9"),
            system_program: pubkey!("11111111111111111111111111111111"),
            token_program: pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            associated_token_program: pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"),
        };

        // Act
        let decoder = MoonshotDecoder;
        let instruction =
            carbon_test_utils::read_instruction("tests/fixtures/migrate_funds_ix.json")
                .expect("read fixture");
        let decoded = decoder
            .decode_instruction(&instruction)
            .expect("decode instruction");
        let decoded_arranged_accounts =
            migrate_funds::MigrateFunds::arrange_accounts(&instruction.accounts)
                .expect("aranage accounts");

        // Assert
        assert_eq!(decoded.data, expected_ix);
        assert_eq!(decoded.accounts, expected_accounts);
        assert_eq!(decoded.program_id, PROGRAM_ID);
        assert_eq!(decoded_arranged_accounts, expected_arranged_accounts);
    }
}
