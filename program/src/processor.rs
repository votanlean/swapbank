use crate::errors::TokenSwapError;
use crate::instruction::TokenSwapIntruction;
use solana_program::decode_error::DecodeError;
use solana_program::program_error::PrintProgramError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};
pub mod initialize;
pub mod swap_sol_to_token;
pub mod swap_token_to_sol;

pub struct Processor {}
impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("process instructions");
        let instruction = TokenSwapIntruction::unpack(instruction_data)?;
        match instruction {
            TokenSwapIntruction::Initialize => {
                msg!("Initialize...");
                initialize::process(program_id, accounts)?;
            }
            TokenSwapIntruction::SwapSolToToken { data } => {
                msg!("Swap token to SOL");
                swap_sol_to_token::process(program_id, accounts, data)?;
            }
            TokenSwapIntruction::SwapTokenToSol { data } => {
                msg!("Swap SOL to token");
                swap_token_to_sol::process(program_id, accounts, data)?;
            }
        }

        Ok(())
    }
}

impl PrintProgramError for TokenSwapError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(&self.to_string());
    }
}
