//! Ethereum Beacon Chain Preset - Max Operations Per Block
//!
//! Upper bounds on the number of operations allowed in a single block.

pub const MAX_PROPOSER_SLASHINGS: u64 = 1 << 4; // 16
pub const MAX_ATTESTER_SLASHINGS: u64 = 1 << 1; // 2
pub const MAX_ATTESTATIONS: u64 = 1 << 7; // 128
pub const MAX_DEPOSITS: u64 = 1 << 4; // 16
pub const MAX_VOLUNTARY_EXITS: u64 = 1 << 4; // 16
