use crate::errors::SwapBankError;
use crate::state;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};

pub fn process(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let vault = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let (vault_pda, vault_bump_seed) =
        Pubkey::find_program_address(&[b"vault", mint.key.as_ref()], program_id);

    if vault_pda != *vault.key {
        msg!("Invalid vault account");
        return Err(SwapBankError::InvalidAccountAddress.into());
    }

    msg!("create vault {} ...", vault.key.to_string());
    invoke_signed(
        &create_account(
            &payer.key,
            &vault.key,
            Rent::get()?.minimum_balance(state::SWAP_BANK_ACCOUNT_LEN),
            state::SWAP_BANK_ACCOUNT_LEN as u64,
            program_id,
        ),
        &[payer.clone(), system_program.clone(), vault.clone()],
        &[&[b"vault", mint.key.as_ref(), &[vault_bump_seed]]],
    )?;

    Ok(())
}
