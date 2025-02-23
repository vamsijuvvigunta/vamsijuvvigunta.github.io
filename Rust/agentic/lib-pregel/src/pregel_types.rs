use std::{collections::HashMap, fmt::Debug, ops::Deref, sync::Arc};
use tokio::sync::Mutex;

use async_trait::async_trait;

pub struct PregelContext {    
}

/// Prgel algorithm send messages into the nodes 
/// but collects messages out from the exec method.
/// Normally I'd expect 
///    exec(inMsg) -> Result<OutMsg>
/// 
/// Since this is done in three phases, it beceoms
///    take_msg(inMsg)
///    exec()-> Result<ExecStatus>
///    deain_out_msg() -> Result<OutMsg>
/// 
/// When not doing at massive scale however, the pregel algo
/// can cache incoming-msgs and outgoing msgs and reduce this 
/// back down to a function call. Trading off node impl simplicity
/// against potentila centralization related performance drops
/// and memory increase.
#[async_trait]
pub trait PregelNode {
    // The impl will define it's own Error and Message types.
    // These should match with the ones defined for the edges
    // as well.    
    type Error : Debug;
    type Message : Debug + Clone;    

    fn name(&self) -> &String;
    fn description(&self) -> &String;

    // Main invocation of the node    
    async fn exec(&self, 
        ctx: Arc<Mutex<PregelContext>>,
        in_msgs : Option<Vec<Self::Message>>) -> std::result::Result<Self::Message, Self::Error>;
}

// Bare thing for now. Likely can add cost of upstream tokens, latency,
// dollar cost etc.
//==================================================================
#[derive(Clone, Debug)]
pub struct PregelPetgraphEdge {
}

impl PregelPetgraphEdge {
    pub fn new() -> Self {
        PregelPetgraphEdge{}
    }
}

//===================================================================
#[derive(Debug)]
pub enum PregelAlgorithmError {
    NoSuchNode(String),
    InternalErrorMissingNode(String)
}

#[derive(Debug)]
pub enum PregelRunStatus {
    Pending,
    MaxSuperStepsReached,
    NormalExecution{num_super_steps: u32},    
    TerminateOnNodeError{num_super_steps: u32},    
}

#[derive(Debug)]
pub struct PregelRunResult<TMsg, TErr, TNodeIndex> {
    pub per_node_result : HashMap<TNodeIndex, 
                                  std::result::Result<Vec<TMsg>, TErr>>,

    // 1 indexed
    pub super_steps_executed: u32,
    pub max_super_steps     : u32,
    pub run_status          : PregelRunStatus
}

impl<TMsg, TErr, TNodeIndex> PregelRunResult<TMsg, TErr, TNodeIndex> {
    pub fn new(max_super_steps:u32) -> Self {
        PregelRunResult {
            per_node_result: HashMap::new(),
            super_steps_executed: 0,
            max_super_steps: max_super_steps,
            run_status: PregelRunStatus::Pending,
        }
    }
}

#[async_trait]
pub trait PregelAlgorithm {
    
    type NodeTraitObj: Send + Sync + ?Sized;
    type BoxedNodeTraitObject : Deref;
    type ArcMutexedBoxedNodeTraitObject;    

    type Error;
    type Message;
    type NodeIndex;
    type EdgeIndex;
    type RunResult;

    // This will take ownership of the node_ptr
    fn add_node(&mut self, node_ptr: Self::BoxedNodeTraitObject) -> Self::NodeIndex;

    // Adds a directed edge between two nodes with an associated edge weight.
    // Takes ownership of the edge trait object. We could take an edge weight as
    // edge_ptr: Self::BoxedEdgeTraitObject, but since I have no idea what it'll contain
    // will not be exposing it in the API and simply use an int internally.
    fn add_edge(&mut self, 
        from_node: Self::NodeIndex, to_node: Self::NodeIndex) -> Self::EdgeIndex;

    // The return has a Mutex to allow async+mutating the shared node ptr.
    fn get_node(&self, idx: & Self::NodeIndex) -> std::result::Result<Self::ArcMutexedBoxedNodeTraitObject, Self::Error>;    

    // Returns Ok(true) on success or an error code.
    fn send_message(&mut self, to: Self::NodeIndex, msg: Self::Message) -> std::result::Result<bool, Self::Error>;

    // Run the main pregel algorithm
    //  Err - Error
    //  Ok  - Collection of output Messages from nodes that have no outgoing edges
    //      - Msgs collected in all but last super-step are discarded
    // Considered implementing this here but error code need actual impl unles I 
    //  want to delegate that to a method as well.
    async fn run_pregel(&mut self, max_super_steps: u32) -> core::result::Result<Self::RunResult, Self::Error>;
}