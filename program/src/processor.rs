use crate::errors::SwapBankError;
use crate::state;
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction::create_account,
    sysvar::Sysvar,
};
pub struct Processor {}
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::{instruction, state::Account as TokenAccount};
impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let payer = next_account_info(accounts_iter)?;
        let swap_bank = next_account_info(accounts_iter)?;
        let mint_a = next_account_info(accounts_iter)?;
        let mint_b = next_account_info(accounts_iter)?;
        let vault_a = next_account_info(accounts_iter)?;
        let vault_b = next_account_info(accounts_iter)?;
        let token_program_id = next_account_info(accounts_iter)?;
        let associated_token_program_id = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;
        let rent_account = next_account_info(accounts_iter)?;

        if !payer.is_signer {
            msg!("payer have to be a signer");
            return Err(SwapBankError::AccountIsNotSigner.into());
        }
        if !swap_bank.is_writable {
            msg!("swap bank account needs to be writable");
            return Err(SwapBankError::AccountIsNotWritable.into());
        }

        let (swap_bank_pda, swap_bank_bump) = Pubkey::find_program_address(
            &[b"swap_bank", mint_a.key.as_ref(), mint_b.key.as_ref()],
            program_id,
        );
        if swap_bank_pda != *swap_bank.key {
            msg!("Invalid swap_bank account");
            return Err(SwapBankError::InvalidAccountAddress.into());
        }

        let (vault_a_pda, vault_a_bump) = Pubkey::find_program_address(
            &[
                b"swap_bank",
                payer.key.as_ref(),
                mint_a.key.as_ref(),
                swap_bank.key.as_ref(),
            ],
            program_id,
        );

        if vault_a_pda != *vault_a.key {
            msg!("Invalid vault_a account");
            return Err(SwapBankError::InvalidAccountAddress.into());
        }
        msg!(
            "vault_a_pda: {}, vault_a: {}",
            vault_a_pda.to_string(),
            vault_a.key.to_string(),
        );

        let (vault_b_pda, vault_b_bump) = Pubkey::find_program_address(
            &[
                b"swap_bank",
                payer.key.as_ref(),
                mint_b.key.as_ref(),
                swap_bank.key.as_ref(),
            ],
            program_id,
        );

        if vault_b_pda != *vault_b.key {
            msg!("Invalid vault_b account");
            return Err(SwapBankError::InvalidAccountAddress.into());
        }
        msg!(
            "vault_b_pda: {}, vault_b: {}",
            vault_b_pda.to_string(),
            vault_b.key.to_string(),
        );

        msg!("create SwapBank");
        invoke_signed(
            &create_account(
                &payer.key,
                &swap_bank.key,
                Rent::get()?.minimum_balance(state::SWAP_BANK_ACCOUNT_LEN),
                state::SWAP_BANK_ACCOUNT_LEN as u64,
                program_id,
            ),
            &[payer.clone(), system_program.clone(), swap_bank.clone()],
            &[&[
                b"swap_bank",
                mint_a.key.as_ref(),
                mint_b.key.as_ref(),
                &[swap_bank_bump],
            ]],
        )?;

        let rent = Rent::get()?.minimum_balance(TokenAccount::LEN);

        msg!("create Vault A");

        invoke_signed(
            &create_account(
                &payer.key,
                &vault_a_pda,
                rent,
                TokenAccount::LEN as u64,
                &token_program_id.key,
            ),
            &[payer.clone(), vault_a.clone(), token_program_id.clone()],
            &[&[
                b"swap_bank",
                payer.key.as_ref(),
                mint_a.key.as_ref(),
                swap_bank.key.as_ref(),
                &[vault_a_bump],
            ]],
        )?;

        let ix = spl_token::instruction::initialize_account(
            &token_program_id.key,
            &vault_a.key,
            &mint_a.key,
            &swap_bank.key,
        )?;
        invoke(
            &ix,
            &[
                vault_a.clone(),
                mint_a.clone(),
                swap_bank.clone(),
                rent_account.clone(),
                token_program_id.clone(),
            ],
        )?;

        msg!("create Vault B");

        invoke_signed(
            &create_account(
                &payer.key,
                &vault_b_pda,
                rent,
                TokenAccount::LEN as u64,
                &token_program_id.key,
            ),
            &[payer.clone(), vault_b.clone(), token_program_id.clone()],
            &[&[
                b"swap_bank",
                payer.key.as_ref(),
                mint_b.key.as_ref(),
                swap_bank.key.as_ref(),
                &[vault_b_bump],
            ]],
        )?;

        let ix = spl_token::instruction::initialize_account(
            &token_program_id.key,
            &vault_b.key,
            &mint_b.key,
            &swap_bank.key,
        )?;
        invoke(
            &ix,
            &[
                vault_b.clone(),
                mint_b.clone(),
                swap_bank.clone(),
                rent_account.clone(),
                token_program_id.clone(),
            ],
        )?;

        // * Allocate data to swapbank
        let swapbank_info = state::SwapBankAccount {
            admin: *payer.key,
            vault_a: *vault_a.key,
            vault_b: *vault_b.key,
        };
        let swapbank_data = &mut *swap_bank.data.borrow_mut();
        swapbank_info.serialize(swapbank_data)?;

        Ok(())
    }
}
