// use borsh::{BorshDeserialize, BorshSerialize};

// #[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]

use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::errors::SwapBankError::InvalidInstruction;

pub enum SwapBankIntruction {
    Initialize,
    Swap { data: u64 },
}

//function of enum
impl SwapBankIntruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        //check instruction type
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        //unpack the rest data for each instruction
        return match tag {
            0 => Ok(Self::Initialize),
            1 => Ok(Self::Swap {
                data: Self::unpack_data(rest)?,
            }),
            _ => Err(InvalidInstruction.into()),
        };
    }

    fn unpack_data(input: &[u8]) -> Result<u64, ProgramError> {
        let data = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;

        return Ok(data);
    }
}
