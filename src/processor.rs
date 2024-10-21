use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo}, address_lookup_table::instruction, entrypoint::{self, ProgramResult,}, msg, pubkey::Pubkey
    };
// pub mod instruction;
use crate::{error::StakingError, instruction::Instruction, state::PoolStorageAccount};    

    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo],instruction_data: &[u8],) -> ProgramResult {

            let instruction = Instruction::try_from_slice(instruction_data)?;

            match instruction {
                Instruction::Initialize { rewards_per_token } =>{
                    msg!("Initialized pool");
                   return process_initialize_pool(program_id, accounts, rewards_per_token);
                },

                _ => {
                    return Err(StakingError::InvalidInstruction.into())
                }
                
            }
        }


        fn process_initialize_pool( program_id: &Pubkey, accounts: &[AccountInfo], rewards_per_token: u64, ) -> ProgramResult 
        {
                let acc_iter = &mut accounts.iter();
                let signer = next_account_info(acc_iter)?;
                if !signer.is_signer {
                   return Err(StakingError::InvalidSigner.into());
                }

                let storage = next_account_info(acc_iter)?;

                if storage.owner != program_id {
                    return Err(StakingError::AlreadyInitialized.into());
                }

                let mut storage_data: PoolStorageAccount = PoolStorageAccount::try_from_slice(&storage.data.borrow())?;


                storage_data.pool_authority = *signer.key;
                storage_data.total_staked = 0u64;
                storage_data.user_count = 0u64;
                storage_data.rewards_per_token = rewards_per_token;
                storage_data.is_initialized = true;

                // storage_data.serialize(&mut &mut storage.data.borrow_mut()[..])?;
                storage_data.try_to_vec()?;
                msg!("Staking pool is initialized {:#?}", storage_data);


                Ok(())
            }    