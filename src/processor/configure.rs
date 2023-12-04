use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};

use crate::{ferror, state::*, utils::*};

pub fn process_configure(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: ConfigureArgs,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let signer_info = next_account_info(account_info_iter)?;
    let config_info = next_account_info(account_info_iter)?;
    let realy = next_account_info(account_info_iter)?;
    let realy_vault = next_account_info(account_info_iter)?;
    let esrealy = next_account_info(account_info_iter)?;
    let esrealy_vault = next_account_info(account_info_iter)?;
    let transfer_auth = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;
    let rent_info = next_account_info(account_info_iter)?;
    let system_info = next_account_info(account_info_iter)?;

    assert_eq_pubkey(&rent_info, &sysvar::rent::id())?;
    assert_eq_pubkey(&system_info, &solana_program::system_program::id())?;
    assert_signer(&signer_info)?;
    let bump = assert_config(&program_id, &config_info)?;
    let realy_vault_bump = assert_realy_vault(program_id, realy, realy_vault)?;
    let esrealy_vault_bump = assert_esrealy_vault(program_id, esrealy, esrealy_vault)?;
    let auth_bump = assert_transfer_authority(program_id, realy, esrealy, transfer_auth)?;
    let authority_seed = [
        program_id.as_ref(),
        realy.key.as_ref(),
        esrealy.key.as_ref(),
        "transfer_auth".as_bytes(),
        &[auth_bump],
    ];
    let mut is_created = true;
    if config_info.data_is_empty() {
        create_or_allocate_account_raw(
            *program_id,
            config_info,
            rent_info,
            system_info,
            signer_info,
            ConfigureData::LEN,
            &[program_id.as_ref(), "config".as_bytes(), &[bump]],
        )?;
        //creat realy vault
        msg!("create realy vault");
        spl_token_create_account(
            &token_program_info,
            &signer_info,
            &realy,
            &realy_vault,
            &transfer_auth,
            &[
                program_id.as_ref(),
                realy.key.as_ref(),
                "realy_vault".as_bytes(),
                &[realy_vault_bump],
            ],
            &authority_seed,
            &rent_info,
        )?;

        //creat esrealy vault
        msg!("create esrealy vault");
        spl_token_create_account(
            &token_program_info,
            &signer_info,
            &esrealy,
            &esrealy_vault,
            &transfer_auth,
            &[
                program_id.as_ref(),
                esrealy.key.as_ref(),
                "esrealy_vault".as_bytes(),
                &[esrealy_vault_bump],
            ],
            &authority_seed,
            &rent_info,
        )?;

        spl_token_mint(
            token_program_info,
            signer_info,
            esrealy,
            esrealy_vault,
            transfer_auth,
            &[],
            &[],
            &authority_seed,
            rent_info,
            args.max_supply,
        )?;

        is_created = false;
    }

    let mut config_data = ConfigureData::from_account_info(config_info)?;

    if is_created {
        if config_data.authority != *signer_info.key {
            return ferror!("invalid authority");
        }
        assert_owned_by(config_info, &program_id)?;
    } else {
        config_data.esrealy = esrealy.key.clone();
    }

    config_data.authority = args.authority;
    config_data.realy = realy.key.clone();
    config_data.max_supply = args.max_supply;
    config_data.serialize(&mut &mut config_info.data.borrow_mut()[..])?;

    Ok(())
}
