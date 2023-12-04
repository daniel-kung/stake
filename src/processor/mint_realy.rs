use crate::{ferror, state::*, utils::*};
use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction, sysvar,
};

pub fn process_mint(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    args: AdminMintArgs,
) -> ProgramResult {
    // let account_info_iter = &mut accounts.iter();
    // let signer_info = next_account_info(account_info_iter)?;
    // let promotion_user_info = next_account_info(account_info_iter)?;
    // let token_account = next_account_info(account_info_iter)?;
    // let pda_creator_info = next_account_info(account_info_iter)?; //nft creator: pda
    // let mint_info = next_account_info(account_info_iter)?;
    // let metadata_info = next_account_info(account_info_iter)?;
    // let edition_info = next_account_info(account_info_iter)?;
    // let collection_mint = next_account_info(account_info_iter)?;
    // let collection_metadata = next_account_info(account_info_iter)?;
    // let collection_master_edition_account = next_account_info(account_info_iter)?;
    // let collection_authority_record = next_account_info(account_info_iter)?;
    // let skc_info = next_account_info(account_info_iter)?;
    // let promotion_info = next_account_info(account_info_iter)?;
    // let collection_info = next_account_info(account_info_iter)?;
    // let charge_info = next_account_info(account_info_iter)?;
    // let metadata_program_info = next_account_info(account_info_iter)?;
    // let token_program_info = next_account_info(account_info_iter)?;
    // let skc_user_info = next_account_info(account_info_iter)?;    
    // let receiver = next_account_info(account_info_iter)?;    
    // let rent_info = next_account_info(account_info_iter)?;
    // let system_info = next_account_info(account_info_iter)?;

    // assert_signer(&signer_info)?;
    // assert_eq_pubkey_0(&rent_info, &sysvar::rent::id())?;
    // assert_eq_pubkey_1(&system_info, &solana_program::system_program::id())?;

    // let mut skc_data = SKCData::from_account_info(skc_info)?;
    // let collection_data = CollectionData::from_account_info(collection_info)?;
    // assert_eq_pubkey_2(&charge_info, &skc_data.char_addr)?;
    // let mut arr_infos = vec![];

    // for _ in skc_data.arr.iter() {
    //     let arr_info = next_account_info(account_info_iter)?;
    //     let arr_data = ArrData::from_account_info(arr_info)?;
    //     if arr_data.token_ids.len() > 0 {
    //         arr_infos.push(arr_info);
    //     }
    // }

    // // mint fee
    // if collection_data.admin != *signer_info.key {
    //     return ferror!("signer is not admin");     
    // }

    // //check sale state
    // if !skc_data.is_open {
    //     return ferror!("sale not open");
    // }
    // // let now_ts = now_timestamp();
    // //check supply
    // if skc_data.sale_num < 1 || arr_infos.len() == 0 || skc_data.total_supply >= skc_data.max_supply
    // {
    //     return ferror!("sold out");
    // }
    // let mut name = skc_data.name.clone();
    // let mut uri = skc_data.base_uri.clone();
    // //deal random mint logic
    // // let arr_index = now_ts % arr_infos.len() as u64;
    // let mut arr_index: u64;
    // let mut arr_data : ArrData;
    // if args.is_merchant == 0 {  // 非mint到商家，则需要进行随机
    //     let now_ts = now_timestamp();
    //     arr_index = now_ts % arr_infos.len() as u64;
    //     arr_data = ArrData::from_account_info(arr_infos[arr_index as usize])?;
    //     if arr_data.token_ids.len() == 1 {
    //         if skc_data.max_supply != 1 {  // max_supply == 1 在下面有处理
    //             uri = uri + "/" + &arr_data.token_ids[0].to_string();     
    //             name = name  + "#" + &arr_data.token_ids[0].to_string();
    //         }
    //         arr_data.token_ids.pop();
    //     } else {
    //         let index = now_ts % arr_data.token_ids.len() as u64;
    //         name = name + "#" + &arr_data.token_ids[index as usize].to_string();
    //         uri = uri + "/" + &arr_data.token_ids[index as usize].to_string();
    //         arr_data.token_ids[index as usize] = arr_data.token_ids[arr_data.token_ids.len() - 1];
    //         arr_data.token_ids.pop();
    //     }
    // } else {
    //     arr_index = 0;
    //     arr_data = ArrData::from_account_info(arr_infos[arr_index as usize])?;
    //     if arr_data.token_ids.len() == 1 {
    //         // name = name + &arr_data.token_ids[0].to_string();
    //         // uri = uri + &arr_data.token_ids[0].to_string();
    //         // arr_data.token_ids.pop();
    //         if skc_data.max_supply != 1 {  // max_supply == 1 在下面有处理
    //             uri = uri + "/" + &arr_data.token_ids[0].to_string();     
    //             name = name  + "#" + &arr_data.token_ids[0].to_string();
    //         }
    //         arr_data.token_ids.pop();
    //     } else {
    //         // let index = now_ts % arr_data.token_ids.len() as u64;
    //         let index = 0;
    //         name = name + "#" + &arr_data.token_ids[index as usize].to_string();
    //         uri = uri  + "/" +  &arr_data.token_ids[index as usize].to_string();
    //         // arr_data.token_ids[index as usize] = arr_data.token_ids[arr_data.token_ids.len() - 1];
    //         // arr_data.token_ids.pop();
    //         arr_data.token_ids.remove(index);   // 永远干掉0
    //     }
    // }

    // let pda_bump = assert_pda_creator(&program_id, collection_mint, pda_creator_info)?;
    // let pda_seed = [
    //     program_id.as_ref(),
    //     collection_mint.key.as_ref(),
    //     "pda_creator".as_bytes(),
    //     &[pda_bump],
    // ];
    // if skc_data.max_supply == 1 {
    //     uri = skc_data.base_uri.clone();
    //     name = skc_data.name.clone();
    // }
    // // check user_info minted number

    // let bump = assert_prom_user_info(&program_id, promotion_info, receiver, promotion_user_info)?;
    // let bump_seed = &[
    //     program_id.as_ref(),
    //     promotion_info.key.as_ref(),
    //     receiver.key.as_ref(),
    //     &[bump],
    // ];
    // if promotion_user_info.data_is_empty() {
    //     create_or_allocate_account_raw(
    //         *program_id,
    //         promotion_user_info,
    //         rent_info,
    //         system_info,
    //         signer_info,
    //         UserData::LEN,
    //         bump_seed,
    //     )?;
    // }

    // let mut promotion_user_data = UserData::from_account_info(promotion_user_info)?;
    // promotion_user_data.minted += 1;
    // promotion_user_data.serialize(&mut *promotion_user_info.try_borrow_mut_data()?)?;

    // // user_skc data
    // let skc_bump = assert_prom_user_info(&program_id, skc_info, receiver, skc_user_info)?;
    // let skc_bump_seed = &[
    //     program_id.as_ref(),
    //     skc_info.key.as_ref(),
    //     receiver.key.as_ref(),
    //     &[skc_bump],
    // ];
    // if skc_user_info.data_is_empty() {
    //     create_or_allocate_account_raw(
    //         *program_id,
    //         skc_user_info,
    //         rent_info,
    //         system_info,
    //         signer_info,
    //         UserData::LEN,
    //         skc_bump_seed,
    //     )?;
    // }

    // let mut skc_user_data = UserData::from_account_info(skc_user_info)?;
    // skc_user_data.minted += 1;
    // skc_user_data.serialize(&mut *skc_user_info.try_borrow_mut_data()?)?;


    // if skc_data.sale_price > 0 && collection_data.admin != *signer_info.key {
    //     let price;
    //     if skc_data.whitelist_start_ts <= now_ts
    //         && now_ts <= skc_data.whitelist_start_ts + skc_data.whitelist_duration
    //     {
    //         price = skc_data.whitelist_sale_price;
    //         //check whitelist
    //         let node = solana_program::keccak::hashv(&[
    //             &args.index.to_le_bytes(),
    //             &signer_info.key.to_bytes(),
    //         ]);
    //         if !merkle_proof_verify(args.proof, skc_data.root, node.0) {
    //             return ferror!("invalid proof");
    //         }
    //         if user_data.minted >= skc_data.whitelist_per_address {
    //             return ferror!("exceed whitelist max per wallet");
    //         }
    //         user_data.minted += 1;
    //         user_data.serialize(&mut *user_info.try_borrow_mut_data()?)?;
    //     } else if skc_data.public_start_ts <= now_ts
    //         && now_ts <= skc_data.public_start_ts + skc_data.public_duration
    //     {
    //         price = skc_data.sale_price;
    //         if user_data.minted >= skc_data.public_per_address {
    //             return ferror!("exceed public max per wallet");
    //         }
    //         user_data.minted += 1;
    //         user_data.serialize(&mut *user_info.try_borrow_mut_data()?)?;
    //     } else {
    //         return ferror!("you are not in whitelist or sale closed");
    //     }
    //     match skc_data.char_token {
    //         Token::Sol => {
    //             invoke(
    //                 &system_instruction::transfer(&signer_info.key, &skc_data.char_addr, price),
    //                 &[
    //                     signer_info.clone(),
    //                     charge_info.clone(),
    //                     system_info.clone(),
    //                 ],
    //             )?;
    //         }
    //         _ => {
    //             spl_token_transfer_invoke(
    //                 token_program_info.clone(),
    //                 token_account.clone(),
    //                 charge_info.clone(),
    //                 signer_info.clone(),
    //                 price,
    //             )?;
    //         }
    //     }
    // }

    //deal creators
    // let mut creators = vec![mpl_token_metadata::state::Creator {
    //     address: *pda_creator_info.key,
    //     verified: true,
    //     share: 0,
    // }];
    // for creator in collection_data.creators.iter() {
    //     creators.push(creator.clone());
    // }

    // //create metadata
    // invoke_signed(
    //     &create_metadata_accounts_v2(
    //         *metadata_program_info.key,
    //         *metadata_info.key,
    //         *mint_info.key,
    //         *signer_info.key,
    //         *signer_info.key,
    //         *pda_creator_info.key,
    //         name,
    //         skc_data.symbol.clone(),
    //         uri,
    //         Some(creators),
    //         collection_data.fee,
    //         true,
    //         true,
    //         Some(Collection {
    //             verified: false,
    //             key: *collection_mint.key,
    //         }),
    //         None,
    //     ),
    //     &[
    //         metadata_info.clone(),
    //         mint_info.clone(),
    //         signer_info.clone(),
    //         metadata_program_info.clone(),
    //         token_program_info.clone(),
    //         system_info.clone(),
    //         rent_info.clone(),
    //         pda_creator_info.clone(),
    //         collection_mint.clone(),
    //     ],
    //     &[&pda_seed],
    // )?;

    // //create edition
    // invoke_signed(
    //     &create_master_edition_v3(
    //         *metadata_program_info.key,
    //         *edition_info.key,
    //         *mint_info.key,
    //         *pda_creator_info.key,
    //         *signer_info.key,
    //         *metadata_info.key,
    //         *signer_info.key,
    //         Some(0),
    //     ),
    //     &[
    //         edition_info.clone(),
    //         mint_info.clone(),
    //         signer_info.clone(),
    //         metadata_info.clone(),
    //         metadata_program_info.clone(),
    //         token_program_info.clone(),
    //         system_info.clone(),
    //         rent_info.clone(),
    //         pda_creator_info.clone(),
    //     ],
    //     &[&pda_seed],
    // )?;

    // //verify collection
    // invoke_signed(
    //     &verify_collection(
    //         *metadata_program_info.key,
    //         *metadata_info.key,
    //         *pda_creator_info.key,
    //         *signer_info.key,
    //         *collection_mint.key,
    //         *collection_metadata.key,
    //         *collection_master_edition_account.key,
    //         Some(*collection_authority_record.key),
    //     ),
    //     &[
    //         collection_mint.clone(),
    //         signer_info.clone(),
    //         metadata_info.clone(),
    //         metadata_program_info.clone(),
    //         token_program_info.clone(),
    //         system_info.clone(),
    //         rent_info.clone(),
    //         collection_metadata.clone(),
    //         collection_master_edition_account.clone(),
    //         collection_authority_record.clone(),
    //         pda_creator_info.clone(),
    //     ],
    //     &[&pda_seed],
    // )?;

    // skc_data.total_supply += 1;
    // skc_data.sale_num -= 1;
    // arr_data.serialize(&mut *arr_infos[arr_index as usize].try_borrow_mut_data()?)?;
    // skc_data.serialize(&mut *skc_info.try_borrow_mut_data()?)?;
    Ok(())
}
