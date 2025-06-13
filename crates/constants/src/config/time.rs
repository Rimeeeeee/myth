//! Ethereum Beacon Chain Preset - Additional Time Parameters
//!
//! Constants related to Eth1 follow distance and validator time-based transitions.

use myth_types::{Epoch, Slot};

pub const SECONDS_PER_SLOT: Slot = 12; // seconds
pub const SECONDS_PER_ETH1_BLOCK: u64 = 14; // seconds
pub const MIN_VALIDATOR_WITHDRAWABILITY_DELAY: Epoch = 1 << 8; // 256 epochs
pub const SHARD_COMMITTEE_PERIOD: Epoch = 1 << 8; // 256 epochs
pub const ETH1_FOLLOW_DISTANCE: u64 = 1 << 11; // 2048 Eth1 blocks
