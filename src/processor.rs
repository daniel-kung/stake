use std::env::args;

use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::{instruction::*, state::UnstakeArgs};

pub mod configure;
pub use configure::*;

pub mod stake;
pub use stake::*;

pub mod unstake;
pub use unstake::*;

pub mod clean;
pub use clean::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = AppInstruction::try_from_slice(input)?;
    match instruction {
        AppInstruction::Configure(args) => {
            msg!("Instruction: Configure");
            process_configure(program_id, accounts, args)
        }
        AppInstruction::Stake(args) => {
            msg!("Instruction: Stake");
            process_stake(program_id, accounts, args)
        }
        AppInstruction::Unstake(args) => {
            msg!("Instruction: Unstake");
            process_unstake(program_id, accounts, args)
        }
        AppInstruction::Clean(args) => {
            msg!("Instruction: Clean");
            process_clean(program_id, accounts, args)
        }
    }
}
