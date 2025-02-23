use std::collections::HashMap;

use crate::pregel_types::PregelAlgorithm;

/// Expanded trait to also include some finer grained methods
/// used during implementation.
/// 
/// Only reason I am doing this is to get around rust's insistance
/// that `impl trait for struct` only contain methods from that trait.
/// If I want to add helper methods, then they have to be in the struct
/// and not in a separate `impl struct` block else where.
pub(crate) trait PregelAlgorithmImpl : PregelAlgorithm {
    type InternalError;    
    type NodeIdMailBoxMap;
    type NodeIdArcMutexdNodeMap;
    type GraphArcMutexdNodes;    

    // For all the nodes deliver messages generated in the previous super-step.
    // This is done first in the super-step
    fn deliver_msgs_along_edges(
        graph: &Self::GraphArcMutexdNodes, 
        id_mbox_map: &mut Self::NodeIdMailBoxMap);

    fn deliver_msgs_along_edges_for_node(
        graph: &Self::GraphArcMutexdNodes, 
        node_id: Self::NodeIndex,
        id_mbox_map: &mut Self::NodeIdMailBoxMap);

    // Get the nodes that will participate in the super-step
    //      - Those that have messages sent to them in a previous super-setep
    //       - If first super-step, then all nodes with no incoling edges.
    fn superstep_node_ids(graph: &Self::GraphArcMutexdNodes, 
        id_mbox_map: &Self::NodeIdMailBoxMap,
        super_step: u32)
        -> Vec<Self::NodeIndex>;   

    // Once msgs are delivered. Any node that has messages left in it's 
    // outbox (i.e. nodes with no outgoing connections. Leaf terminals) will have 
    // their msgs cleared out. This is done since I am considering the output of 
    // pregel to be the final outbox messages in each node. So only retaining them
    // for the latest super-step.
    fn clear_disconnected_outboxes(id_mbox_map: &mut Self::NodeIdMailBoxMap);
    
    fn take_node_input_msgs(node_ids: &Vec<Self::NodeIndex>, id_mbox_map: &mut Self::NodeIdMailBoxMap) 
        -> HashMap<Self::NodeIndex, Option<Vec<Self::Message>>>;    

    fn into_run_results(&mut self,
        max_supersteps : u32,
        final_superstep: u32,
        node_errors: &mut Vec<(Self::NodeIndex, Self::Error)>) -> core::result::Result<Self::RunResult, Self::Error>;

    // Helpers
    fn deliver_msgs_to_nodes(msgs: Vec<Self::Message>, 
            target_nodes: Vec<Self::NodeIndex>, 
            id_mbox_map: &mut Self::NodeIdMailBoxMap);
}