use std::mem::size_of;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize)]
pub struct Vault {
    pub admin: Pubkey,
    pub vault: Pubkey,
    pub mint: Pubkey,
}

pub const TOKEN_SWAP_ACCOUNT_LEN: usize = size_of::<Pubkey>() * 3;
