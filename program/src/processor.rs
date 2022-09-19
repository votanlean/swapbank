use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};
pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!(
            "program_id: {}, accounts: {}, instruction_data: {:?}",
            program_id,
            accounts.len(),
            instruction_data
        );
        Ok(())
    }
}
