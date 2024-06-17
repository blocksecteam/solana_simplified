use crate::instructions::init::IndexAccountData;
use crate::instructions::post::PostArticleData;
use borsh::BorshDeserialize;
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::msg;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;

pub fn list_articles(program_id: &Pubkey, accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    // parse AccountInfo
    let accounts_iter = &mut accounts.iter();
    let _payer = next_account_info(accounts_iter)?;
    let index_pda = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;
    let article_pdas: Vec<&AccountInfo> = accounts_iter.collect();

    // check the address and then read data from index_pda
    let (index_pda_key, _) = Pubkey::find_program_address(&["INDEX_PDA".as_bytes()], program_id);
    assert_eq!(index_pda.key, &index_pda_key);
    let index_data: IndexAccountData =
        IndexAccountData::try_from_slice(&index_pda.data.borrow()[..])?;

    // check whether the length of the article pda vector equals to current index
    if article_pdas.len() != index_data.cur_index as usize {
        return Err(ProgramError::InvalidArgument);
    }

    // evaluate the address of each article pda
    for i in 0..index_data.cur_index {
        let (article_pda_key, _) =
            Pubkey::find_program_address(&["ARTICLE_PDA".as_bytes(), &i.to_le_bytes()], program_id);
        assert_eq!(article_pdas[i as usize].key, &article_pda_key);
    }

    // it's time to list all the posted articles
    for i in 0..article_pdas.len() {
        let post_article_data: PostArticleData =
            PostArticleData::try_from_slice(&article_pdas[i].data.borrow()[..])?;
        msg!(
            "Article index: {}\n, title: {}\n, content: {}\n\n",
            i,
            post_article_data.title,
            post_article_data.content
        );
    }

    Ok(())
}
