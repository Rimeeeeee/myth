//! Beacon State Dependencies for Ethereum Consensus.

use crate::misc::{BeaconBlockHeader, Checkpoint, Eth1Data, Fork, PendingAttestation, Validator};
use alloy_primitives::FixedBytes;
use myth_constants::{
    EPOCHS_PER_HISTORICAL_VECTOR, EPOCHS_PER_SLASHINGS_VECTOR, Gwei, Root, Slot,
    misc::JUSTIFICATION_BITS_LENGTH,
    preset::{
        EPOCHS_PER_ETH1_VOTING_PERIOD, HISTORICAL_ROOTS_LIMIT, MAX_ATTESTATIONS, SLOTS_PER_EPOCH,
        SLOTS_PER_HISTORICAL_ROOT, VALIDATOR_REGISTRY_LIMIT,
    },
};
use r_ssz::{BitVector, fixed_vectors::FixedVector};
///
/// See: <https://github.com/ethereum/consensus-specs/blob/dev/specs/phase0/beacon-chain.md#beacon-state>
#[derive(Debug)]
pub struct BeaconState {
    pub genesis_time: u64,
    pub genesis_validators_root: Root,
    pub slot: Slot,
    pub fork: Fork,
    pub latest_block_header: BeaconBlockHeader,
    pub block_roots: FixedVector<Root, { SLOTS_PER_HISTORICAL_ROOT as usize }>,
    pub state_roots: FixedVector<Root, { SLOTS_PER_HISTORICAL_ROOT as usize }>,
    pub historical_roots: [Root; HISTORICAL_ROOTS_LIMIT as usize],
    pub eth1_data: Eth1Data,
    pub eth1_data_votes: [Eth1Data; { EPOCHS_PER_ETH1_VOTING_PERIOD * SLOTS_PER_EPOCH } as usize],
    pub eth1_deposit_index: u64,
    pub validators: [Validator; VALIDATOR_REGISTRY_LIMIT as usize],
    pub balances: [Gwei; VALIDATOR_REGISTRY_LIMIT as usize],
    pub randao_mixes: FixedVector<FixedBytes<32>, { EPOCHS_PER_HISTORICAL_VECTOR as usize }>,
    pub slashings: FixedVector<Gwei, { EPOCHS_PER_SLASHINGS_VECTOR as usize }>,
    pub previous_epoch_attestations:
        [PendingAttestation; { MAX_ATTESTATIONS * SLOTS_PER_EPOCH } as usize],
    pub current_epoch_attestations:
        [PendingAttestation; { MAX_ATTESTATIONS * SLOTS_PER_EPOCH } as usize],
    pub justification_bits: BitVector<{ JUSTIFICATION_BITS_LENGTH as usize }>,
    pub previous_justified_checkpoint: Checkpoint,
    pub current_justified_checkpoint: Checkpoint,
    pub finalized_checkpoint: Checkpoint,
}
