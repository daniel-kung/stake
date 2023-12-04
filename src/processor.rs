use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::*;

pub mod configure;
pub use configure::*;

pub mod mint_esrealy;
pub use mint_esrealy::*;

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
        AppInstruction::MintEsrealy(args) => {
            msg!("Instruction: MintEsrealy");
            process_mint_esrealy(program_id, accounts, args)
        }
    }
}
