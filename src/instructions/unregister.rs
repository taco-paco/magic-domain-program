use borsh::{BorshDeserialize, BorshSerialize};
use solana::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct UnregisterInstruction(pub Pubkey);
