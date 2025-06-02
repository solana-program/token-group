//! Program entrypoint

use {
    crate::processor,
    solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey},
    solana_program_error::ToStr,
    spl_token_group_interface::error::TokenGroupError,
};

solana_program::entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    if let Err(error) = processor::process(program_id, accounts, instruction_data) {
        msg!(error.to_str::<TokenGroupError>());
        return Err(error);
    }
    Ok(())
}
