use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    program::invoke, pubkey::Pubkey,
};

use crate::errors::TokenSwapError;
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    msg!("amount: {}", amount);

    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let payer_ata = next_account_info(accounts_iter)?;
    let program = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let vault = next_account_info(accounts_iter)?;
    let vault_ata = next_account_info(accounts_iter)?;
    let token_program_id = next_account_info(accounts_iter)?;

    msg!("transfer {} Token lamports from payer to vault", amount);
    let payer_token_to_vault_ix = spl_token::instruction::transfer(
        &token_program_id.key,
        &payer_ata.key,
        &vault_ata.key,
        &payer.key,
        &[],
        amount,
    )?;
    invoke(
        &payer_token_to_vault_ix,
        &[
            token_program_id.clone(),
            payer_ata.clone(),
            vault_ata.clone(),
            program.clone(),
            payer.clone(),
        ],
    )?;

    msg!("transfer SOL from vault to payer");
    **vault.try_borrow_mut_lamports()? -= amount / 10 as u64;
    **payer.try_borrow_mut_lamports()? += amount / 10 as u64;
    msg!(
        "{} SOL lamports transferred from vault to payer",
        amount / 10 as u64,
    );

    Ok(())
}
