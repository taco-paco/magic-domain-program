use borsh::{BorshDeserialize, BorshSerialize};

use crate::state::validator_info::ValidatorInfo;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct RegisterInstruction(pub ValidatorInfo);
