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
pub mod swap;

pub struct Processor {}
impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("process instructions");
        let instruction = SwapBankIntruction::try_from_slice(instruction_data).map_err(|err| {
            msg!("invalid instruction data. cause {:}", err);
            ProgramError::InvalidInstructionData
        })?;
        msg!("instruction: {:?}", instruction);
        match instruction {
            SwapBankIntruction::Initialize {} => {
                msg!("Initialize Swap Bank");
                initialize::process(&program_id, &accounts)?;
            }
            SwapBankIntruction::Swap { amount } => {
                msg!("Swap");
                swap::process(program_id, accounts, amount)?;
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
