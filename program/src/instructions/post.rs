use crate::instructions::init::IndexAccountData;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;

const TITLE_LENGTH_LIMIT: usize = 20;
const CONTENT_LENGTH_LIMIT: usize = 1000;

pub fn post_article(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data_inner: &[u8],
) -> Result<(), ProgramError> {
    // extract AccountInfo
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let index_pda = next_account_info(accounts_iter)?;
    let new_article_pda = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;

    // check the address and then read data from index_pda
    let (index_pda_key, _) = Pubkey::find_program_address(&["INDEX_PDA".as_bytes()], program_id);
    assert_eq!(index_pda.key, &index_pda_key);
    let mut index_data: IndexAccountData =
        IndexAccountData::try_from_slice(&index_pda.data.borrow()[..])?;

    // check the address of new_article_pda
    let (article_pda_key, bump) = Pubkey::find_program_address(
        &[
            "ARTICLE_PDA".as_bytes(),
            &index_data.cur_index.to_le_bytes(),
        ],
        program_id,
    );
    assert_eq!(new_article_pda.key, &article_pda_key);

    // deserialize and check the posted article data
    let post_article_data: PostArticleData =
        PostArticleData::try_from_slice(instruction_data_inner)?;
    if post_article_data.title.len() > TITLE_LENGTH_LIMIT
        || post_article_data.content.len() > CONTENT_LENGTH_LIMIT
    {
        return Err(ProgramError::InvalidArgument);
    }

    // Everything is Ok! Create the account and apply the changes
    let account_span = (post_article_data.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            &new_article_pda.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[payer.clone(), new_article_pda.clone()],
        &[&[
            "ARTICLE_PDA".as_bytes(),
            &index_data.cur_index.to_le_bytes(),
            &[bump],
        ]],
    )?;

    post_article_data.serialize(&mut &mut new_article_pda.data.borrow_mut()[..])?;
    index_data.cur_index += 1;
    index_data.serialize(&mut &mut index_pda.data.borrow_mut()[..])?;

    Ok(())
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct PostArticleData {
    pub title: String,
    pub content: String,
}
