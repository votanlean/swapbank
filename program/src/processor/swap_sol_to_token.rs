use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
use spl_associated_token_account::solana_program::system_instruction;

use crate::errors::SwapBankError;
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    msg!("swap sol to token, lamports: {}", lamports);
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let payer_ata = next_account_info(accounts_iter)?;
    let program = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let vault = next_account_info(accounts_iter)?;
    let vault_ata = next_account_info(accounts_iter)?;
    let token_program_id = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    let (vault_pda, vault_bump_seed) =
        Pubkey::find_program_address(&[b"vault", mint.key.as_ref()], program_id);
    if vault_pda != *vault.key {
        msg!("Invalid vault account");
        return Err(SwapBankError::InvalidAccountAddress.into());
    }

    msg!("transfer SOL from payer to program");
    let take_sol_from_payer_ix = system_instruction::transfer(payer.key, vault.key, lamports);
    let required_accounts_take_sol = [system_program.clone(), payer.clone(), vault.clone()];
    invoke(&take_sol_from_payer_ix, &required_accounts_take_sol);

    //send token
    msg!("transfer token from vault_ata to payer_ata");
    let give_token_to_payer_ix = spl_token::instruction::transfer(
        token_program_id.key,
        &vault_ata.key,
        &payer_ata.key,
        &vault.key,
        &[],
        lamports * 10 as u64,
    )?;
    invoke_signed(
        &give_token_to_payer_ix,
        &[
            token_program_id.clone(),
            payer_ata.clone(),
            vault_ata.clone(),
            vault.clone(),
        ],
        &[&[b"vault", mint.key.as_ref(), &[vault_bump_seed]]],
    )?;

    Ok(())
}
