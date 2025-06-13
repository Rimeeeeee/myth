//! Misc Constants

use myth_types::{Epoch, Slot};

pub const UINT64_MAX: u64 = u64::MAX;
pub const UINT64_MAX_SQRT: u64 = 4_294_967_295;
pub const GENESIS_SLOT: Slot = 0;
pub const GENESIS_EPOCH: Epoch = 0;
pub const FAR_FUTURE_EPOCH: Epoch = u64::MAX;
pub const BASE_REWARDS_PER_EPOCH: u64 = 4;
pub const DEPOSIT_CONTRACT_TREE_DEPTH: u64 = 32;
pub const JUSTIFICATION_BITS_LENGTH: u64 = 4;
pub const ENDIANNESS: &str = "little";
