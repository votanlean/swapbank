use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError, program_error::PrintProgramError, program_error::ProgramError,
};
use thiserror::Error;

#[derive(Error, Debug, Clone, Copy, FromPrimitive)]
pub enum SwapBankError {
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("Invalid Account address.")]
    InvalidAccountAddress,
    #[error("Account is not writable")]
    AccountIsNotWritable,
    #[error("Account is not signer")]
    AccountIsNotSigner,
    #[error("Invalid Mint")]
    InvalidMint,
    #[error("Unique Mint")]
    UniqueMintAccounts,
}

impl From<SwapBankError> for ProgramError {
    fn from(e: SwapBankError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for SwapBankError {
    fn type_of() -> &'static str {
        "SwapBank error"
    }
}
