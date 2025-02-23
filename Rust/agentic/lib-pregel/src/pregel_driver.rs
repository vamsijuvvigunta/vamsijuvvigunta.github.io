use std::sync::Arc;
use tokio::sync::Mutex;

use async_trait::async_trait;

use crate::pregel_impl::pregel_petgraph::PregelPetgraph;
use crate::pregel_types::{
    PregelNode,
    PregelRunResult,
    PregelAlgorithm,
    PregelAlgorithmError};

pub struct PregelDriver<TMsg, TErr>
where
    TMsg: Send + Sync + Clone + 'static,
    TErr: Send + Sync + From<PregelAlgorithmError> + 'static,
{
    algo_impl: PregelPetgraph<TMsg, TErr>,
}

impl<TMsg, TErr> PregelDriver<TMsg, TErr>
where
    TMsg: Send + Sync + Clone + std::fmt::Debug + 'static,
    TErr: Send + Sync + From<PregelAlgorithmError> + std::fmt::Debug + 'static,
{
    pub fn new() -> PregelDriver<TMsg, TErr> {
        PregelDriver::<TMsg, TErr> {
            algo_impl : PregelPetgraph::<TMsg, TErr>::new()
        }
    }
}

//-----------------------------------------
#[async_trait]
impl<TMsg, TErr> PregelAlgorithm for PregelDriver<TMsg, TErr>
where
    TMsg: Send + Sync + Clone + std::fmt::Debug + 'static,
    TErr: Send + Sync + From<PregelAlgorithmError> + std::fmt::Debug + 'static,
{
    // concretize all the associated types.
    type Error = TErr;
    type Message = TMsg;
    type NodeIndex = petgraph::stable_graph::NodeIndex;
    type EdgeIndex = petgraph::stable_graph::EdgeIndex;
    type RunResult = PregelRunResult<Self::Message, Self::Error, Self::NodeIndex>;
    type NodeTraitObj = dyn PregelNode<Error = TErr, Message = TMsg> + Send + Sync;
    type BoxedNodeTraitObject = Box<Self::NodeTraitObj>;
    type ArcMutexedBoxedNodeTraitObject = Arc<Mutex<Self::BoxedNodeTraitObject>>;    

    fn add_node(
        &mut self,
        node: Self::BoxedNodeTraitObject,
    ) -> Self::NodeIndex {
      self.algo_impl.add_node(node)
    }
    
    fn get_node(
        &self,
        idx: &Self::NodeIndex,
    ) -> std::result::Result<Self::ArcMutexedBoxedNodeTraitObject, Self::Error> {
        self.algo_impl.get_node(idx)
    }

    fn add_edge(&mut self, 
        from_node: Self::NodeIndex, to_node: Self::NodeIndex
    ) -> Self::EdgeIndex {
            self.algo_impl.add_edge(from_node, to_node)
    }

    fn send_message(
        &mut self,
        to: Self::NodeIndex,
        msg: Self::Message,
    ) -> std::result::Result<bool, Self::Error> {
        self.algo_impl.send_message(to, msg)
    }

    async fn run_pregel(&mut self, max_super_steps: u32) -> core::result::Result<Self::RunResult, Self::Error> {    
        self.algo_impl.run_pregel(max_super_steps).await
    }
}
