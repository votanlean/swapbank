use borsh::BorshDeserialize;
use solana_program::system_program;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::mem;
use tokenswap::entrypoint::process_instruction;

#[tokio::test]
async fn test_initialize() {
    let program_id = Pubkey::new_unique();
    let mint = Pubkey::new_unique();
    let (vault, _bump_seed) =
        Pubkey::find_program_address(&[b"vault", &mint.to_bytes()], &program_id);

    let mut program_test = ProgramTest::new(
        "tokenswap", // Run the BPF version with `cargo test-bpf`
        program_id,
        processor!(process_instruction), // Run the native version with `cargo test`
    );

    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

    // vault does not exist
    assert_eq!(
        banks_client.get_account(vault).await.expect("get_account"),
        None,
    );

    let mut transaction = Transaction::new_with_payer(
        &[Instruction::new_with_bincode(
            program_id,
            &[0], // ignored but makes the instruction unique in the slot
            vec![
                AccountMeta::new(payer.pubkey(), true),
                AccountMeta::new(vault, false),
                AccountMeta::new(mint, false),
                AccountMeta::new(system_program::id(), false),
            ],
        )],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // vault created successfully on the system
    assert_ne!(
        banks_client.get_account(vault).await.expect("get_account"),
        None,
    );
}
