use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
};
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], amount: f64) -> ProgramResult {
    msg!("amount: {}", amount);

    Ok(())
}
