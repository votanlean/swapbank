use crate::errors::SwapBankError;
use crate::instruction;
use crate::instruction::SwapBankIntruction;
use crate::state;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::decode_error::DecodeError;
use solana_program::program_error::{PrintProgramError, ProgramError};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};
pub mod initialize;
pub mod swap_sol;
pub mod utils;

pub struct Processor {}
impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("process instructions");
        let instruction = SwapBankIntruction::unpack(instruction_data)?;
        match instruction {
            SwapBankIntruction::Initialize => {
                msg!("Initialize Swap Bank");
                initialize::process(&program_id, &accounts)?;
            }
            SwapBankIntruction::Swap { data } => {
                msg!("Swap");
                swap_sol::process(program_id, accounts, data)?;
            }
        }

        Ok(())
    }
}

impl PrintProgramError for SwapBankError {
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
