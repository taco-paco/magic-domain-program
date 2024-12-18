use std::net::SocketAddrV4;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

use crate::{consts::VALIDATOR_INFO_SEED, state::features::FeaturesSet, ID};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SyncInfoInstruction {
    pub identity: Pubkey,
    pub addr: Option<SocketAddrV4>,
    pub block_time_ms: Option<u16>,
    pub fees: Option<u16>,
    pub features: Option<FeaturesSet>,
}

impl SyncInfoInstruction {
    pub fn pda(&self) -> Pubkey {
        let seeds = [VALIDATOR_INFO_SEED, self.identity.as_ref()];
        Pubkey::find_program_address(&seeds, &ID).0
    }
}
