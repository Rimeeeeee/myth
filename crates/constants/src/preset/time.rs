//! Ethereum Beacon Chain Preset - Time Parameters
//!
//! Constants representing durations in slots or epochs for various consensus operations.

use crate::{Epoch, Slot};

pub const MIN_ATTESTATION_INCLUSION_DELAY: Slot = 1; // slots
pub const SLOTS_PER_EPOCH: Slot = 1 << 5; // 32 slots
pub const MIN_SEED_LOOKAHEAD: Epoch = 1; // epochs
pub const MAX_SEED_LOOKAHEAD: Epoch = 1 << 2; // 4 epochs
pub const MIN_EPOCHS_TO_INACTIVITY_PENALTY: Epoch = 1 << 2; // 4 epochs
pub const EPOCHS_PER_ETH1_VOTING_PERIOD: Epoch = 1 << 6; // 64 epochs
pub const SLOTS_PER_HISTORICAL_ROOT: Slot = 1 << 13; // 8192 slots
