use crate::instructions::init::init;
use crate::instructions::list::list_articles;
use crate::instructions::post::post_article;
use solana_program::program_error::ProgramError;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let (instruction_discriminant, instruction_data_inner) = instruction_data.split_at(1);
    match instruction_discriminant[0] {
        0 => init(program_id, accounts)?,
        1 => post_article(program_id, accounts, instruction_data_inner)?,
        2 => list_articles(program_id, accounts)?,
        _ => {
            return Err(ProgramError::InvalidInstructionData);
        }
    }

    Ok(())
}
