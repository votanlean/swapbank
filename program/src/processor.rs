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
pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Create tokenswap account...");
        let accounts_iter = &mut accounts.iter();
        let payer = next_account_info(accounts_iter)?;
        let swap_bank = next_account_info(accounts_iter)?;
        let mint_a = next_account_info(accounts_iter)?;
        let mint_b = next_account_info(accounts_iter)?;
        let system_program = next_account_info(accounts_iter)?;

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
            msg!("Invalid swap_bank_account");
            return Err(SwapBankError::InvalidAccountAddress.into());
        }

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

        Ok(())
    }
}
