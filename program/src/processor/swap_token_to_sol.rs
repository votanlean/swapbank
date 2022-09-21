use solana_program::{
    account_info::next_account_info, account_info::AccountInfo, entrypoint::ProgramResult, msg,
    program::invoke, program_pack::Pack, pubkey::Pubkey,
};

use crate::errors::TokenSwapError;

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

    // let vault_data = spl_token::state::Account::unpack(&vault.data.borrow())?;
    // if &vault_data.mint != mint.key {
    //     msg!("Invalid mint token");
    //     return Err(TokenSwapError::InvalidMint.into());
    // }

    msg!("transfer {} Token lamports from payer to vault", lamports);
    let payer_token_to_vault_ix = spl_token::instruction::transfer(
        &token_program_id.key,
        &payer_ata.key,
        &vault_ata.key,
        &payer.key,
        &[],
        lamports,
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
    **vault.try_borrow_mut_lamports()? -= lamports / 10 as u64;
    **payer.try_borrow_mut_lamports()? += lamports / 10 as u64;
    msg!(
        "{} SOL lamports transferred from vault to payer",
        lamports / 10 as u64,
    );

    Ok(())
}
