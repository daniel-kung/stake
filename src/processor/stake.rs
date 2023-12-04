use crate::{state::*, utils::*};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    sysvar,
};

pub fn process_stake(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: StakeArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let pay_token_account = next_account_info(account_info_iter)?;
    let receiver = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let transfer_auth = next_account_info(account_info_iter)?;
    let realy_vault = next_account_info(account_info_iter)?;
    let esrealy_vault = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_config(&program_id, &config_info)?;
    assert_signer(&signer_info)?;
    assert_eq_pubkey_0(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey_1(&system_info, &solana_program::system_program::id())?;

    let mut config_data = ConfigureData::from_account_info(config_info)?;

    assert_eq_pubkey(realy_vault, &config_data.realy_vault)?;
    assert_eq_pubkey(esrealy_vault, &config_data.esrealy_vault)?;

    //transfer realy to vault
    msg!("transfer realy to vault");
    spl_token_transfer_invoke(
        token_program_info.clone(),
        pay_token_account.clone(),
        realy_vault.clone(),
        signer_info.clone(),
        args.amt,
    )?;

    //transfer esrealy to user
    msg!("transfer out esrealy");
    spl_token_transfer_invoke(
        token_program_info.clone(),
        esrealy_vault.clone(),
        receiver.clone(),
        transfer_auth.clone(),
        args.amt,
    )?;

    config_data.supply += args.amt;
    config_data.serialize(&mut &mut config_info.data.borrow_mut()[..])?;

    Ok(())
}
