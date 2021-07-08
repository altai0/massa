#![feature(vecdeque_binary_search)]
#![feature(bool_to_option)]
#![feature(hash_drain_filter)]
#![feature(map_first_last)]

#[macro_use]
extern crate logging;

mod block_graph;
mod config;
mod consensus_controller;
mod consensus_worker;
mod error;
mod ledger;
mod random_selector;
mod timeslots;
pub use block_graph::BootsrapableGraph;
pub use block_graph::{
    BlockGraphExport, DiscardReason, ExportCompiledBlock, ExportDiscardedBlocks, LedgerDataExport,
    Status,
};
pub use config::ConsensusConfig;
pub use consensus_controller::{
    start_consensus_controller, ConsensusCommandSender, ConsensusEventReceiver, ConsensusManager,
};
pub use consensus_worker::{ConsensusCommand, ConsensusEvent};
pub use error::ConsensusError;
pub use ledger::{LedgerData, LedgerExport};
pub use timeslots::{
    get_block_slot_timestamp, get_current_latest_block_slot, get_latest_block_slot_at_timestamp,
};
#[cfg(test)]
mod tests;
