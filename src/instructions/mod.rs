use borsh::{BorshDeserialize, BorshSerialize};
use register::RegisterInstruction;
use sync::SyncInfoInstruction;
use unregister::UnregisterInstruction;

pub mod register;
pub mod sync;
pub mod unregister;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum Instruction {
    Register(RegisterInstruction),
    Unregister(UnregisterInstruction),
    SyncInfo(SyncInfoInstruction),
}
