use solana_program::{
    account_info::next_account_info,
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
};
use spl_associated_token_account::solana_program::{system_instruction, system_program};
use spl_token::instruction::transfer;

use crate::processor::utils;

use crate::errors::SwapBankError;
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], lamports: u64) -> ProgramResult {
    msg!("swap sol to token, lamports: {}", lamports);
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    // let payer_ata = next_account_info(accounts_iter)?;
    // let program = next_account_info(accounts_iter)?;
    // let mint = next_account_info(accounts_iter)?;
    let vault = next_account_info(accounts_iter)?;
    // let vault_ata = next_account_info(accounts_iter)?;
    // let token_program_id = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;

    // let (vault_pda, vault_bump_seed) = Pubkey::find_program_address(
    //     &[b"vault", program.key.as_ref(), mint.key.as_ref()],
    //     program_id,
    // );
    // if vault_pda != *vault.key {
    //     msg!("Invalid vault account");
    //     return Err(SwapBankError::InvalidAccountAddress.into());
    // }
    // let vault_seeds = &[
    //     b"vault",
    //     program.key.as_ref(),
    //     mint.key.as_ref(),
    //     &[vault_bump_seed],
    // ];

    msg!("transfer SOL from payer to program");
    let take_sol_ix = system_instruction::transfer(payer.key, vault.key, lamports);
    let required_accounts_take_sol = [system_program.clone(), payer.clone(), vault.clone()];
    invoke(&take_sol_ix, &required_accounts_take_sol);

    Ok(())
}
