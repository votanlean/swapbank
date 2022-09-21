use std::mem::size_of;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct SwapBankAccount {
    pub admin: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
}

pub const TOKEN_SWAP_ACCOUNT_LEN: usize = size_of::<Pubkey>() * 3;
