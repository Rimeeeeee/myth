//! Ethereum Beacon Chain Preset - Genesis Settings
//!
//! Constants related to the initialization of the beacon chain.

use alloy_primitives::FixedBytes;
use myth_types::Version;

pub const MIN_GENESIS_ACTIVE_VALIDATOR_COUNT: u64 = 1 << 14; // 16,384
pub const MIN_GENESIS_TIME: u64 = 1606824000; // Dec 1, 2020, 12pm UTC
pub const GENESIS_FORK_VERSION: Version = FixedBytes([0, 0, 0, 0]);
pub const GENESIS_DELAY: u64 = 604_800; // 7 days in seconds
