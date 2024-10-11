use carbon_core::deserialize::CarbonDeserialize;
use carbon_core::instruction::InstructionDecoder;
use carbon_macros::try_decode_instructions;

use super::PumpDecoder;
pub mod buy;
pub mod complete_event;
pub mod create;
pub mod create_event;
pub mod initialize;
pub mod sell;
pub mod set_params;
pub mod set_params_event;
pub mod trade_event;
pub mod withdraw;

#[derive(
    carbon_proc_macros::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum PumpInstruction {
    Initialize(initialize::Initialize),
    SetParams(set_params::SetParams),
    Create(create::Create),
    Buy(buy::Buy),
    Sell(sell::Sell),
    Withdraw(withdraw::Withdraw),
    CreateEvent(create_event::CreateEvent),
    TradeEvent(trade_event::TradeEvent),
    CompleteEvent(complete_event::CompleteEvent),
    SetParamsEvent(set_params_event::SetParamsEvent),
}

impl<'a> InstructionDecoder<'a> for PumpDecoder {
    type InstructionType = PumpInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_sdk::instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        try_decode_instructions!(instruction,
            PumpInstruction::Initialize => initialize::Initialize,
            PumpInstruction::SetParams => set_params::SetParams,
            PumpInstruction::Create => create::Create,
            PumpInstruction::Buy => buy::Buy,
            PumpInstruction::Sell => sell::Sell,
            PumpInstruction::Withdraw => withdraw::Withdraw,
            PumpInstruction::CreateEvent => create_event::CreateEvent,
            PumpInstruction::TradeEvent => trade_event::TradeEvent,
            PumpInstruction::CompleteEvent => complete_event::CompleteEvent,
            PumpInstruction::SetParamsEvent => set_params_event::SetParamsEvent,
        )
    }
}