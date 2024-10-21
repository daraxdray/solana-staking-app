use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::program_error::ProgramError;
use std::convert::TryInto;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub enum Instruction {
    Initialize {
    rewards_per_token: u64,
    },
    CreateUser {
    
    },
    Stake {
    amount: u64
    },
    Unstake {
    amount: u64
    },
    Claim{
    amount: u64},
    }



    impl Instruction {
        pub fn unpack(input: &[u8]) -> Result<_,ProgramError>{

            let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

            match tag {
                0 => return Ok(Instruction::Initialize { rewards_per_token: 0 }),
                1 => return Ok(Instruction::CreateUser {  }),
                2 => {
                    let val = rest[..4].try_into()?;
                    match val {
                        Ok(i) => return Ok(Instruction::Stake {amount:0  }),
                        _ => return Err(ProgramError::InvalidInstructionData)
                    }
                    
                },
                _ => return Err(ProgramError::InvalidInstructionData)
            }


            Ok(())
        }
    }