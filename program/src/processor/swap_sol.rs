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
pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    msg!("amount: {}", amount);

    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let swap_bank = next_account_info(accounts_iter)?;
    let swap_bank_ata = next_account_info(accounts_iter)?;
    let mint_a = next_account_info(accounts_iter)?;
    let mint_b = next_account_info(accounts_iter)?;
    let vault_a = next_account_info(accounts_iter)?;
    let vault_b = next_account_info(accounts_iter)?;
    let token_program_id = next_account_info(accounts_iter)?;
    let payerAta = next_account_info(accounts_iter)?;
    let to_token_account = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let program = next_account_info(accounts_iter)?;
    let programAta = next_account_info(accounts_iter)?;
    // * checks
    if !payer.is_signer {
        msg!("authority needs to have signer privilege");
        return Err(SwapBankError::AccountIsNotSigner.into());
    }

    if !to_token_account.is_writable {
        msg!("receiving token account needs to be writable");
        return Err(SwapBankError::AccountIsNotWritable.into());
    }

    if !payerAta.is_writable {
        msg!("from token account needs to be writable");
        return Err(SwapBankError::AccountIsNotWritable.into());
    }

    if !vault_a.is_writable {
        msg!("vault A is not writable");
        return Err(SwapBankError::AccountIsNotWritable.into());
    }

    if !vault_b.is_writable {
        msg!("vailt B is not writable");
        return Err(SwapBankError::AccountIsNotWritable.into());
    }

    let to_token_account_data = spl_token::state::Account::unpack(&to_token_account.data.borrow())?;
    let from_token_account_data = spl_token::state::Account::unpack(&payerAta.data.borrow())?;

    if &from_token_account_data.mint != mint_a.key {
        msg!("sending token account is not of the same mint as token A");
        return Err(SwapBankError::InvalidMint.into());
    }

    // if &to_token_account_data.mint != mint_b.key {
    //     msg!("receving token account is not of the same mint as token B");
    //     return Err(SwapBankError::InvalidMint.into());
    // }

    // if to_token_account_data.mint == from_token_account_data.mint {
    //     msg!("receiving token account cannot be of the same mint as the sending token account");
    //     return Err(SwapBankError::UniqueMintAccounts.into());
    // }

    let (swap_bank_pda, swap_bank_bump) = Pubkey::find_program_address(
        &[b"swap_bank", mint_a.key.as_ref(), mint_b.key.as_ref()],
        program_id,
    );
    if swap_bank_pda != *swap_bank.key {
        msg!("Invalid swap_bank account");
        return Err(SwapBankError::InvalidAccountAddress.into());
    }

    // * Exchange
    // send
    let token_a_b_xr: u64 = 10;
    let amount_a: u64 = utils::amount_to_lamports(mint_a, amount).unwrap();
    let amount_b: u64 = utils::amount_to_lamports(mint_b, amount * token_a_b_xr).unwrap();

    msg!(
        "transfer amount: {} from {} to vault A {}",
        amount_a,
        payerAta.key.to_string(),
        to_token_account.key.to_string(),
    );
    let deposit_into_a_ix = spl_token::instruction::transfer(
        &token_program_id.key,
        &payerAta.key,
        &to_token_account.key,
        &payer.key,
        &[&payer.key],
        amount_a,
    )
    .unwrap();

    invoke(
        &deposit_into_a_ix,
        &[
            token_program_id.clone(),
            payerAta.clone(),
            vault_a.clone(),
            payer.clone(),
            to_token_account.clone(),
        ],
    )
    .unwrap();

    msg!("transfer token from programPDAAta to payerATA");
    let vault_token_to_payer_ix = spl_token::instruction::transfer(
        &token_program_id.key,
        &swap_bank_ata.key,
        &payerAta.key,
        &swap_bank.key,
        &[],
        501 * 10e8 as u64,
    )?;
    invoke_signed(
        &vault_token_to_payer_ix,
        &[
            token_program_id.clone(),
            payerAta.clone(),
            swap_bank_ata.clone(),
            swap_bank.clone(),
        ],
        &[&[
            b"swap_bank",
            mint_a.key.as_ref(),
            mint_b.key.as_ref(),
            &[swap_bank_bump],
        ]],
    )?;

    msg!("transfer token from payer to program");
    let payer_token_to_vault_ix = spl_token::instruction::transfer(
        &token_program_id.key,
        &payerAta.key,
        &programAta.key,
        &payer.key,
        &[],
        502 * 10e8 as u64,
    )?;
    invoke(
        &payer_token_to_vault_ix,
        &[
            token_program_id.clone(),
            payerAta.clone(),
            programAta.clone(),
            program.clone(),
            payer.clone(),
        ],
    )?;

    msg!("transfer SOL from payer to program");
    let collect_sol_ix = system_instruction::transfer(payer.key, swap_bank.key, 1 * 10e8 as u64);
    invoke(
        &collect_sol_ix,
        &[system_program.clone(), payer.clone(), swap_bank.clone()],
    );

    msg!("transfer SOL from program to payer");

    **swap_bank.try_borrow_mut_lamports()? -= 1 * 10e8 as u64;
    **payer.try_borrow_mut_lamports()? += 1 * 10e8 as u64;
    msg!(
        "{} lamports transferred from vault to payer",
        1 * 10e8 as u64,
    );

    Ok(())
}
