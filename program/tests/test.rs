#![cfg(feature = "test-bpf")]

use std::assert_eq;

use {
    solana_program::{pubkey::Pubkey, system_program},
    solana_program_test::ProgramTest,
    solana_sdk::signature::{Keypair, Signer},
};

#[test]
fn test_swaptoken() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::default();
    program_test.add_program("swaptoken", program_id, None);

    let auth = Keypair::new();
    program_test.add_account(
        auth.pubkey(),
        solana_sdk::account::Account {
            lamports: 100_000_000_000,
            data: vec![],
            owner: system_program::id(),
            ..solana_sdk::account::Account::default()
        },
    );
}
