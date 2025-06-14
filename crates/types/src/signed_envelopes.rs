//! Signed Envelopes Dependencies for Ethereum Consensus.

use crate::{
    beacon_blocks::BeaconBlock, beacon_operations::VoluntaryExit, misc::BeaconBlockHeader,
};
use myth_constants::BLSSignature;
///
/// See: <https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#signed-envelopes>

#[derive(Debug)]
pub struct SignedVoluntaryExit {
    pub message: VoluntaryExit,
    pub signature: BLSSignature,
}

#[derive(Debug)]
pub struct SignedBeaconBlock {
    pub message: BeaconBlock,
    pub signature: BLSSignature,
}

#[derive(Debug)]
pub struct SignedBeaconBlockHeader {
    pub message: BeaconBlockHeader,
    pub signature: BLSSignature,
}
