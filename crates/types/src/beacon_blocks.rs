//! Beacon Blocks Dependencies for Ethereum Consensus.

use crate::{
    beacon_operations::{Attestation, AttesterSlashing, Deposit, ProposerSlashing},
    misc::Eth1Data,
    signed_envelopes::SignedVoluntaryExit,
};
use alloy_primitives::FixedBytes;
use myth_constants::{
    BLSSignature, Root, Slot, ValidatorIndex,
    preset::{
        MAX_ATTESTATIONS, MAX_ATTESTER_SLASHINGS, MAX_DEPOSITS, MAX_PROPOSER_SLASHINGS,
        MAX_VOLUNTARY_EXITS,
    },
};
///
/// See: <https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#beacon-blocks>

#[derive(Debug)]
pub struct BeaconBlockBody {
    pub randao_reveal: BLSSignature,
    pub eth1_data: Eth1Data,
    pub graffiti: FixedBytes<32>,
    pub proposer_slashings: [ProposerSlashing; MAX_PROPOSER_SLASHINGS as usize],
    pub attester_slashings: [AttesterSlashing; MAX_ATTESTER_SLASHINGS as usize],
    pub attestations: [Attestation; MAX_ATTESTATIONS as usize],
    pub deposits: [Deposit; MAX_DEPOSITS as usize],
    pub voluntary_exits: [SignedVoluntaryExit; MAX_VOLUNTARY_EXITS as usize],
}

#[derive(Debug)]
pub struct BeaconBlock {
    pub slot: Slot,
    pub proposer_index: ValidatorIndex,
    pub parent_root: Root,
    pub state_root: Root,
    pub body: BeaconBlockBody,
}
