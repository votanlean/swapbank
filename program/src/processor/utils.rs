use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, msg, program_error::ProgramError, program_pack::Pack, pubkey::Pubkey,
};
pub fn amount_to_lamports(mint: &AccountInfo, amount: u64) -> Result<u64, ProgramError> {
    let mint_account_data = spl_token::state::Mint::unpack_from_slice(&mint.try_borrow_data()?)?;
    let mint_decimals = mint_account_data.decimals;

    let lamports = amount * u64::pow(10, mint_decimals.into());
    Ok(lamports)
}
