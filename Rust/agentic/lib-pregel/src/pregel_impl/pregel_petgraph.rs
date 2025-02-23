use async_trait::async_trait;
use itertools::{Either, Itertools};
use petgraph::prelude::StableDiGraph;
use petgraph::Direction::{Incoming, Outgoing};
use petgraph::visit::{Dfs, EdgeRef};

use tokio::task::JoinSet;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, trace, warn};

use crate::pregel_mailbox::PregelMailbox;
use crate::pregel_types::{
    PregelAlgorithm, PregelAlgorithmError, PregelContext, PregelNode, PregelPetgraphEdge,
    PregelRunResult, PregelRunStatus,
};

use crate::pregel_impl::pregel_impl_types::PregelAlgorithmImpl;

pub(crate) struct PregelPetgraph<TMsg, TErr>
where
    TMsg: Send + Clone,
    TErr: Send    
{
    // We want to share the node impls between the two containers
    //
    // Outermost `Arc` since it is shared and async
    //   Inner Mutex since we want to mutably borrow for operations
    //     Since we are dealing with trait oobjects implemented by
    //     different types.Use Box<dyn PregelNode> as the inner most.
    // rest is setting up PregelNode's associated types.
    id_node_map: HashMap<
        petgraph::stable_graph::NodeIndex,
        Arc<Mutex<
            Box<dyn PregelNode<Error = TErr, Message = TMsg> + Send + Sync>
            >>,
    >,

    graph: StableDiGraph<
        Arc<Mutex<
            Box<dyn PregelNode<Error = TErr, Message = TMsg> + Send + Sync>
            >>,
        PregelPetgraphEdge,
    >,

    id_mbox_map: HashMap<petgraph::stable_graph::NodeIndex, PregelMailbox<TMsg>>,
}

//--------------------------------------------------------------------
impl<TMsg, TErr> PregelPetgraph<TMsg, TErr>
where
    TMsg: Send + Sync + Clone + std::fmt::Debug + 'static,
    TErr: Send + Sync + std::fmt::Debug + From<PregelAlgorithmError> + 'static,
{
    pub fn new() -> PregelPetgraph<TMsg, TErr> {
        PregelPetgraph {
            id_node_map: HashMap::new(),      
            graph: StableDiGraph::new(),
            id_mbox_map: HashMap::new(),
        }
    }    
}

//--------------------------------------------------------------------
// Implement PregelAlgorithm
#[async_trait]
impl<TMsg, TErr> PregelAlgorithm for PregelPetgraph<TMsg, TErr>
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
        // Create a Rc and clone it into both containers
        // Make the contained object a RefCell as we do need to mutate it.
        // Since we take ownership of the box, no danger of it
        // being mutable from the outside
        let node_ref = Arc::new(Mutex::new(node));

        let id = self.graph.add_node(node_ref.clone());
        self.id_node_map.insert(id, node_ref);

        id
    }

    fn get_node(
        &self,
        idx: &Self::NodeIndex,
    ) -> std::result::Result<Self::ArcMutexedBoxedNodeTraitObject, Self::Error> {
        match self.graph.node_weight(idx.clone()) {
            Some(node) => Ok(node.clone()),
            None => Err(
                // Note the TErr : From<PregelAlgoithmError> constraint so we
                // can simply call .into() on PregelAlgorithmError
                PregelAlgorithmError::NoSuchNode("{idx:?}".to_string()).into(),
            ),
        }
    }

    fn add_edge(&mut self, 
        from_node: Self::NodeIndex, to_node: Self::NodeIndex) -> Self::EdgeIndex {
            self.graph.add_edge(from_node, to_node, PregelPetgraphEdge::new())
    }

    fn send_message(
        &mut self,
        to: Self::NodeIndex,
        msg: Self::Message,
    ) -> std::result::Result<bool, Self::Error> {
        // id check and error out?
        if !self.id_node_map.contains_key(&to) {
            Err(PregelAlgorithmError::NoSuchNode("{id:?}".to_string()).into())
        } else {
            // init mbox on demand
            self.id_mbox_map
                .entry(to)
                .or_insert_with(|| PregelMailbox::<TMsg>::new())
                .inbox
                .push_back(msg);

            Ok(true)
        }
    }

    async fn run_pregel(
        &mut self,
        max_supersteps: u32,
    ) -> core::result::Result<Self::RunResult, Self::Error> {
        let node_ctx = Arc::new(Mutex::new(PregelContext {}));

        // Not just final result but we'll use this to store the errors as well.        
        let mut node_errors = Vec::<(Self::NodeIndex, Self::Error)>::new();        

        //-- Begin Super-step loop --------------------------------
        let superstep = 0;
        for superstep in 0..max_supersteps {            

            // Given the outbox of each node (Outputs from previous
            // super-step), drain & move the meesage to inbox of each
            //  outgoing connection's target node
            Self::deliver_msgs_along_edges(&self.graph, &mut self.id_mbox_map);

            // get super-step node-ids and terminate if none     
            // This expects msg delivery to already have happened and simply looks 
            // at nodes that pending msgs (unless it is super-step 0 in which case 
            // all roots are chosen)            
            let ss_ids = Self::superstep_node_ids(&self.graph, &self.id_mbox_map, superstep);
            if ss_ids.len() == 0 {
                debug!("Terminating superstep loop at superstep: {superstep}. No active nodes!)");
                break;
            }
            debug!("Nodes in super-step:{superstep} -> [{ss_ids:?}]");

            // All outputs that are connected via edges to other nodes
            //  would have been transferred to their invox at the end of
            //  prev super-step
            //
            // Those of terminal nodes that have no outgoings will be
            // cleared out here and lost. Maybe later we can decide to
            // push this elsewhere instead of losing them            
            Self::clear_disconnected_outboxes(&mut self.id_mbox_map);

            // take input messages for the nodes in this superstep 
            // these will be drained from the id_mbox_map to be sent to 
            // each node on exec.
            let mut input_msg_map = Self::take_node_input_msgs(&ss_ids, &mut self.id_mbox_map);            

            // 1 - partition into errors and async closures
            // Since we are sending out (id,..), moving id out via drain
            let (node_exec_fns, unresolved_nodes):(Vec<_>, Vec<_>) = ss_ids
                .iter()   
                .map(|id| id.clone())             
                .partition_map(|id| {
                    if let Some(node) = self.graph.node_weight(id) {

                        let input_msgs = input_msg_map
                            .remove_entry(&id).map(|(_,v)| v)
                            .flatten();                    

                        debug!("------------------------\n");
                        debug!("Exec for {id:?} with Msgs=\n{input_msgs:?}\n--------------");
                                                
                        // Cloning all these so aync_fn closute can take ownership
                        // any better way of doing this ?
                        let id = id.clone();
                        let node = node.clone();
                        let node_ctx = node_ctx.clone();

                        debug!("Generating closure for Node:{id:?} in super-step:{superstep} with msgs: {input_msgs:?}");

                        // First move is to ask the Closure to take ownership
                        // async move does what ?
                        let async_fn = move || async move {         
                            debug!("Waiting for lock in Node:{id:?}/ExecClosure");
                            let node_inner = node.lock().await;
                            debug!("Calling Node:{id:?}/ExecClosure");

                            node_inner.exec(node_ctx, input_msgs)
                            .await
                            .map(|r| (id, r))
                            .map_err(|e| (id, e))
                            .inspect(|_| debug!("Returning from Node:{id:?}/ExecClosure"))
                        };

                        Either::Left((id, async_fn))
                } else {                                        
                    Either::Right(id)
                }});

            // 2- Error out on unresolved nodes. 
            //    These are internal errors and means something went seriously wrong.
            //    Note that none of the futures will execute till we await them, so 
            //    as of this line, ss_futures will be idle.
            if unresolved_nodes.len() > 0 {
                error!("Internal error: NodeID:{unresolved_nodes:?} were not resolved to nodes in superstep {superstep}!");
                Err(PregelAlgorithmError::InternalErrorMissingNode(
                    format!("Unexpected internal error. Node resolution failed for {unresolved_nodes:?}!!").to_string()
                    ).into())?;
            }                        

            // 3 - Get all futures scheduled
            //     for now the aborts are left alone.
            // NOTE: Per tokio docs, the spawned future will get executed immediately
            //       even if you don't await the join handle.
            let mut node_join_set = JoinSet::new();
            let _abort_handles:Vec<_> = node_exec_fns
                .into_iter()                
                .map(|(id, f)|
                    (id, node_join_set.spawn(f()))
                )
                .collect();



            // 4 - process each future as it completes.
            while let Some(join_res) = node_join_set.join_next().await {
                debug!("One node-exec completed");
                match join_res {
                    Ok(node_res) => {
                        match node_res {
                            Ok((id,msg)) => {

                                debug!("Node {id:?}'s exec task returned with Success!");
                                trace!("Node {id:?}'s Return Msg=\n---\n{msg:?}\n---\n");

                                // Push into the node's outbox
                                self.id_mbox_map
                                    .entry(id)
                                    .or_insert_with(|| PregelMailbox::<TMsg>::new())
                                    .outbox
                                    .push_back(msg);
                            }
                            Err((id, err)) => {
                                // save errors to decide loop
                                // termination
                                warn!("Node {id:?}'s exec task failed with ExecutionError: {err:?}!");
                                node_errors.push((id, err));
                            }
                        }
                    },
                    Err(je) => {
                        // FIXME: How best to handle this ?
                        // Likely needs termination of the whole thing.
                        error!("JoinError returned for task completion: {je:?}");
                    }
                }                
            }            

            // Termination on node execution errors.
            if node_errors.len() > 0 {
                break;
            }
        }
        //------------------------------- End Super-step loop --        

        self.into_run_results(max_supersteps, superstep, &mut node_errors)
    }
}

impl<TErr, TMsg> PregelAlgorithmImpl for PregelPetgraph<TMsg, TErr>
where
    TMsg: Send + Sync + Clone + std::fmt::Debug + 'static,
    TErr: Send + Sync + From<PregelAlgorithmError> + std::fmt::Debug + 'static,
{
    type InternalError          = PregelAlgorithmError;
    type NodeIdMailBoxMap       = HashMap<Self::NodeIndex, PregelMailbox<TMsg>>;
    type NodeIdArcMutexdNodeMap = HashMap<Self::NodeIndex, Self::ArcMutexedBoxedNodeTraitObject>;
    type GraphArcMutexdNodes    = StableDiGraph<Self::ArcMutexedBoxedNodeTraitObject, PregelPetgraphEdge>;        

    fn superstep_node_ids(graph: &Self::GraphArcMutexdNodes, 
        id_mbox_map: &Self::NodeIdMailBoxMap,
        super_step: u32) -> Vec<Self::NodeIndex> {

        // select all nodes with messages pending in their queue.
        // Either msgs sent from upstream nodes
        // -or-
        // Init msg
        // Once iteration starts, this will mean that Node Outboxes will have to be delivered along 
        // edges before this method is called.
        let active_nodes = id_mbox_map
            .iter()
            .filter(|(_k, v)| v.inbox.len() > 0)
            .map(|(k, _v)| k.clone())
            .collect_vec();

        // At the start only active nodes are those which have init messages sent to them.
        if super_step == 0 {
            if active_nodes.len() > 0 {
                // Those with InitMessages
                debug!("Have {} nodes with init messages. Marking them as active for step==0", active_nodes.len());
                active_nodes
            }
            else {
                debug!("No Nodes with init msgs in step=0. Selecting Externals with no Incoming as start nodes");
                graph
                .externals(Incoming)
                .collect_vec()
            }            
        } else {
            active_nodes
        }
    }

    fn take_node_input_msgs(node_ids: &Vec<Self::NodeIndex>, id_mbox_map: &mut Self::NodeIdMailBoxMap) 
        -> HashMap<Self::NodeIndex, Option<Vec<Self::Message>>>
    {
        let input_msg_map : HashMap<_,_> = node_ids.iter().map(|id| {
            let input_msgs = id_mbox_map
            .get_mut(id)
            .map(|mbox| {
                mbox.inbox.drain(..).collect_vec()
            });

            debug!("Taking message: Node {id:?} ← [[{input_msgs:?}]]");
            (*id, input_msgs)
        }).collect();

        input_msg_map
    }
    
    // Each node that has messages in it's outbox, sends them along it's outgoing edges    
    fn deliver_msgs_along_edges(graph: &Self::GraphArcMutexdNodes, id_mbox_map: &mut Self::NodeIdMailBoxMap)
    {
        // Start with graphs that have no incoming edges. These are the roots        
        let mut start_nodes = graph.externals(petgraph::Direction::Incoming).collect_vec();

        // FIXME: Should we always add roots ? Atleast one root condition!?
        // The basic Prompter -> Tool -> Prompter is a loop with no nodes.
        // Pick any one thing for now.
        if start_nodes.len() == 0 {            
            start_nodes.push(graph.node_indices().next().unwrap());
            trace!("Have no externals. Picking one node to start DFS from: {:?}", start_nodes);
        }

        for start in start_nodes {
            let mut dfs = Dfs::new(graph, start);

            while let Some(node_id) = dfs.next(graph) {
                trace!("Visiting Node[{:?}] in BFS", node_id);
                Self::deliver_msgs_along_edges_for_node(graph, node_id, id_mbox_map);                
            }
        }
    }    

    fn deliver_msgs_along_edges_for_node(graph: &Self::GraphArcMutexdNodes, 
        node_id: Self::NodeIndex,
        id_mbox_map: &mut Self::NodeIdMailBoxMap) 
    {
        // Send messages only (filter for this) if there are output messages waiting.
        // AND we have outgoing edge. Clone the msgs before sending, try 
        // to optimize the clones.
        // 
        // If no outgoing edges, the outbox message is retained and will become 
        // the output of the grap for this super-step.                
        id_mbox_map
            .get_mut(&node_id)
            .filter(|mbox| mbox.outbox.len() > 0)
            .map(|mbox| {
                let target_nodes:Vec<_> = graph.edges_directed(node_id, Outgoing)
                    .map(|edge_ref| edge_ref.target())
                    .collect_vec();                        

                // Only drain if there are outgoing nodes. Otw we need to preserve
                // outgoing msgs since they are part of the graph's output at the end of 
                // each super-step.
                if target_nodes.len() > 0 {
                    debug!("{node_id:?} has outgoing msgs to {} nodes! Sending", target_nodes.len());
                    Some((target_nodes, mbox.outbox.drain(..).collect_vec()))                        
                } else {
                    None
                }
            })
            .flatten()
            .map(|(nodes, msgs)|{
                Self::deliver_msgs_to_nodes(msgs, nodes, id_mbox_map);
            });
    }

    // Sends a list of messages to a collection of nodes
    // The incoming message vec is consumed
    // Each target node gets a cloned copy of the message-list and the messages 
    // are deposited in their inbox
    fn deliver_msgs_to_nodes(msgs: Vec<Self::Message>, 
        target_nodes: Vec<Self::NodeIndex>, 
        id_mbox_map: &mut Self::NodeIdMailBoxMap) {        

        // Make N-1 copies and take/consume msg_vec as the Nth copy.
        let mut cloned_msgs = (1..target_nodes.len())
            .map(|_| msgs.clone())
            .collect_vec();
        cloned_msgs.push(msgs);

        // Deliver them to the inboxes along edges
        // Init mailboxes as needed (see `or_insert_with`)
        target_nodes
            .iter()
            .zip(cloned_msgs.drain(..))
            .for_each(|(target_node_id, mut msg_vec)| {                
                debug!("Delivering {} messages to dest:{target_node_id:?}", msg_vec.len());
                debug!("Messages: {msg_vec:?}");
                id_mbox_map
                    .entry(target_node_id.clone())
                    .or_insert_with(|| PregelMailbox::<TMsg>::new())
                    .inbox.extend(msg_vec.drain(..));                    
            });
    }

    fn clear_disconnected_outboxes(id_mbox_map: &mut Self::NodeIdMailBoxMap) {
        debug!("Clearing out disconnected outbox contents");

        // Don't actually have to look for disconnected ones
        // anything with a mbox can be clered. ALl those connected
        // will already have moved outbox to inbox along the edge.
        id_mbox_map.iter_mut().for_each(|(id, mbox)| {
            if mbox.outbox.len() > 0 {
                debug!("Clearing outbox msgs for node#{id:?}");
                mbox.outbox.clear();
            }
        });
    }

    fn into_run_results(
        &mut self,
        max_supersteps: u32,
        final_superstep: u32,
        node_errors:&mut Vec<(Self::NodeIndex, Self::Error)>,
    ) -> core::result::Result<Self::RunResult, Self::Error> {
        let mut result =
            PregelRunResult::<Self::Message, Self::Error, Self::NodeIndex>::new(max_supersteps);

        // drain outputs of each nodes                
        self.id_mbox_map
        .iter_mut()
        .map(|(id, mbox)| (id, mbox.outbox.drain(..).collect_vec()))        
        .for_each(|(id, msg_vec)| {
            result.per_node_result.insert(
                *id, 
                Ok(msg_vec));
            }
        );
        
        // Move node errors into results and finalize return status
        if node_errors.len() > 0 {
            debug!("The following nodes have errored out of exec: {node_errors:?}");
            warn!("Terminating loop at superstep:{final_superstep} out of a max of {max_supersteps}");

            result.run_status = PregelRunStatus::TerminateOnNodeError {
                num_super_steps: final_superstep + 1,
            };

            node_errors
            .drain(..)                        
            .for_each(|(id, err)| {
                result.per_node_result.insert(id, Err(err));
            });
            
        } else if final_superstep >= max_supersteps {
            result.run_status = PregelRunStatus::MaxSuperStepsReached;
        } else {
            result.run_status = PregelRunStatus::NormalExecution {
                num_super_steps: final_superstep,
            };
        }

        // this does not error out.
        // run_pregel errors out with internal errors on some 
        // conditions. If it reaches here, node errors are encapsulated
        // in the result.
        Ok(result)
    }
}
