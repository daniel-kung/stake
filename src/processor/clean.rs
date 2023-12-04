use crate::{state::*, utils::*};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    pubkey::Pubkey,
    sysvar,
};

pub fn process_clean(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: UnstakeArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let receiver = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let transfer_auth = next_account_info(account_info_iter)?;
    let realy = next_account_info(account_info_iter)?;
    let realy_vault = next_account_info(account_info_iter)?;
    let esrealy = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_config(&program_id, &config_info)?;
    assert_signer(&signer_info)?;
    assert_eq_pubkey_0(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey_1(&system_info, &solana_program::system_program::id())?;
    assert_eq_pubkey_2(&token_program_info, &spl_token::id())?;
    let mut config_data = ConfigureData::from_account_info(config_info)?;
    let auth_bump = assert_transfer_authority(program_id, realy, esrealy, transfer_auth)?;
    let authority_seed = [
        program_id.as_ref(),
        realy.key.as_ref(),
        esrealy.key.as_ref(),
        "transfer_auth".as_bytes(),
        &[auth_bump],
    ];
    assert_eq_pubkey(realy_vault, &config_data.realy_vault)?;
    assert_eq_pubkey(realy, &config_data.realy)?;
    assert_eq_pubkey(esrealy, &config_data.esrealy)?;
    assert_eq_pubkey(signer_info, &config_data.authority)?;

    //transfer realy to user
    spl_token_transfer(
        token_program_info.clone(),
        realy_vault.clone(),
        receiver.clone(),
        transfer_auth.clone(),
        args.amt,
        &authority_seed,
    )?;
    
    config_data.supply -= args.amt;
    config_data.serialize(&mut &mut config_info.data.borrow_mut()[..])?;

    Ok(())
}
