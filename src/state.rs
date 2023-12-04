use borsh::{BorshDeserialize, BorshSchema, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    borsh0_10::try_from_slice_unchecked,
    program_error::ProgramError,
    pubkey::Pubkey, program_pack::Pack,
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, BorshSchema, Clone, Debug, Default, PartialEq)]
pub struct ConfigureArgs {
    /// Contract admin
    pub authority: Pubkey, 
    pub max_supply: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshSchema, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct ConfigureData {
    /// Contract admin
    pub authority: Pubkey,
    pub realy: Pubkey,
    pub realy_vault: Pubkey,
    pub esrealy: Pubkey,
    pub esrealy_vault: Pubkey,
    pub max_supply: u64,
    pub supply: u64,
}

impl ConfigureData {
    pub const LEN: usize = 32 + 32 + 32 + 32 + 32 + 8 + 8;

    pub fn from_account_info(a: &AccountInfo) -> Result<ConfigureData, ProgramError> {
        if a.data_len() != Self::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        try_from_slice_unchecked(&a.data.borrow_mut()).map_err(|_| ProgramError::InvalidAccountData)
    }
}


#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct MintEsrealyArgs {
    pub amt: u64,
}

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Default, PartialEq)]
pub struct BackToRealyArgs {
    pub amt: u64,
}