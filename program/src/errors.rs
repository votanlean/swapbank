use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError, program_error::PrintProgramError, program_error::ProgramError,
};
use thiserror::Error;

#[derive(Error, Debug, Clone, Eq, Copy, FromPrimitive, PartialEq)]
pub enum SwapBankError {
    #[error("Invalid Account address.")]
    InvalidAccountAddress,
    #[error("Account is not writable")]
    AccountIsNotWritable,
    #[error("Account is not signer")]
    AccountIsNotSigner,
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
