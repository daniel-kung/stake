use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_program,
    sysvar::rent,
};

use crate::state::*;

#[repr(C)]
#[derive(Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq)]
pub enum AppInstruction {
    Configure(ConfigureArgs),
    Stake(StakeArgs),
    Unstake(UnstakeArgs)
}

pub fn configure(
    program_id: &Pubkey,
    siger: &Pubkey,
    config_info: &Pubkey,
    realy: &Pubkey,
    realy_vault: &Pubkey,
    esrealy: &Pubkey,
    esrealy_vault: &Pubkey,
    transfer_auth:  &Pubkey,
    args: ConfigureArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*config_info, false),
        AccountMeta::new(*realy, false),
        AccountMeta::new(*realy_vault, false),
        AccountMeta::new(*esrealy, false),
        AccountMeta::new(*esrealy_vault, false),
        AccountMeta::new(*transfer_auth, false),
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(rent::id(), false),
        AccountMeta::new_readonly(system_program::id(), false),
    ];

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::Configure(args).try_to_vec().unwrap(),
    })
}

pub fn stake_realy(
    program_id: &Pubkey,
    siger: &Pubkey,
    pay_token_account: &Pubkey, //realy token account address, spl--token account
    receiver: &Pubkey,
    config_info: &Pubkey,  
    transfer_auth: &Pubkey,
    realy_vault: &Pubkey,
    esrealy_vault: &Pubkey,
    args:StakeArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*pay_token_account, false),
        AccountMeta::new(*receiver, false), 
        AccountMeta::new(*config_info, false), 
        AccountMeta::new_readonly(*transfer_auth, false),
        AccountMeta::new(*realy_vault, false), 
        AccountMeta::new(*esrealy_vault, false), 
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(rent::id(), false),  
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::Stake(args).try_to_vec().unwrap(),
    })
}

pub fn unstake_realy(
    program_id: &Pubkey,
    siger: &Pubkey,
    pay_token_account: &Pubkey, //esrealy token account address, spl--token account
    receiver: &Pubkey,
    config_info: &Pubkey, 
    transfer_auth: &Pubkey,
    realy_vault: &Pubkey,
    esrealy_vault: &Pubkey,   
    args:UnstakeArgs,
) -> Result<Instruction, ProgramError> {
    let accounts = vec![
        AccountMeta::new(*siger, true),
        AccountMeta::new(*pay_token_account, false),
        AccountMeta::new(*receiver, false), 
        AccountMeta::new(*config_info, false), 
        AccountMeta::new_readonly(*transfer_auth, false),
        AccountMeta::new(*realy_vault, false), 
        AccountMeta::new(*esrealy_vault, false), 
        AccountMeta::new_readonly(spl_token::id(), false),
        AccountMeta::new_readonly(rent::id(), false),  
        AccountMeta::new_readonly(system_program::id(), false),
    ];
    
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data: AppInstruction::Unstake(args).try_to_vec().unwrap(),
    })
}