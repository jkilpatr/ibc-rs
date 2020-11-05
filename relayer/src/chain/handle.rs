use std::str::FromStr;
use std::time::Duration;

use crossbeam_channel as channel;
use thiserror::Error;

use ibc::ics02_client::client_def::{AnyClientState, AnyConsensusState, AnyHeader};
use ibc::ics24_host::{identifier::ChainId, Path, IBC_QUERY_PATH};
use ibc::Height;

use tendermint::abci::Path as ABCIPath;
use tendermint::net;
use tendermint_rpc::HttpClient;

use crate::config::ChainConfig;
use crate::foreign_client::ForeignClient;
use crate::msgs::{Datagram, EncodedTransaction, IBCEvent, Packet};
use crate::util::block_on;

use super::{error::ChainError, Chain, Subscription};

/// Implementation of handle specific for Cosmos SDK chains
pub mod cosmos;

mod prod;
pub use prod::ProdChainHandle;

/// Inputs that a Handle may send to a Runtime.
pub enum HandleInput {
    Terminate(channel::Sender<()>),
    Subscribe(channel::Sender<Subscription>),
    GetHeader(Height, channel::Sender<AnyHeader>),
}

pub trait ChainHandle: Send {
    fn id(&self) -> ChainId;

    fn subscribe(&self, chain_id: ChainId) -> Result<Subscription, ChainError>;

    fn query(&self, path: Path, height: Height, prove: bool) -> Result<Vec<u8>, ChainError>;

    fn get_header(&self, height: Height) -> Result<AnyHeader, ChainError>;

    fn get_minimal_set(&self, from: Height, to: Height) -> Result<Vec<AnyHeader>, ChainError>;

    /// Submits a transaction.
    fn submit(&self, transaction: EncodedTransaction) -> Result<(), ChainError>;

    fn get_height(&self, client: &ForeignClient) -> Result<Height, ChainError>;

    fn create_packet(&self, event: IBCEvent) -> Result<Packet, ChainError>;

    /// Given a header originating from this chain, constructs a client state.
    fn assemble_client_state(&self, header: &AnyHeader) -> Result<AnyClientState, ChainError>;

    /// Given a header originating from this chain, constructs a consensus state.
    fn assemble_consensus_state(&self, header: &AnyHeader)
        -> Result<AnyConsensusState, ChainError>;
}
