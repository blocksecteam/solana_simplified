use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::account_info::{next_account_info, AccountInfo};
use solana_program::program::invoke_signed;
use solana_program::program_error::ProgramError;
use solana_program::pubkey::Pubkey;
use solana_program::rent::Rent;
use solana_program::system_instruction;
use solana_program::sysvar::Sysvar;

pub fn init(program_id: &Pubkey, accounts: &[AccountInfo]) -> Result<(), ProgramError> {
    // extract AccountInfo
    let accounts_iter = &mut accounts.iter();
    let payer = next_account_info(accounts_iter)?;
    let index_pda = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;

    // check the address of index_pda
    let (index_pda_key, bump) = Pubkey::find_program_address(&["INDEX_PDA".as_bytes()], program_id);
    assert_eq!(index_pda.key, &index_pda_key);

    // calculate the minimum lamports to deploy
    let index_data = IndexAccountData { cur_index: 0 };
    let account_span = (index_data.try_to_vec()?).len();
    let lamports_required = (Rent::get()?).minimum_balance(account_span);

    // create the account
    invoke_signed(
        &system_instruction::create_account(
            payer.key,
            &index_pda.key,
            lamports_required,
            account_span as u64,
            program_id,
        ),
        &[payer.clone(), index_pda.clone()],
        &[&["INDEX_PDA".as_bytes(), &[bump]]],
    )?;

    // write the data
    index_data.serialize(&mut &mut index_pda.data.borrow_mut()[..])?;

    Ok(())
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct IndexAccountData {
    pub cur_index: u32,
}
