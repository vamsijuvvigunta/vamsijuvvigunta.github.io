use async_trait::async_trait;
use std::{result::Result, sync::Arc};
use tokio::sync::Mutex;
use tracing::debug;
use std::marker::PhantomData;

use lib_pregel::pregel_types::{PregelContext, PregelNode};

use crate::agentic_errors::AgenticError;

//-- Helper Types ---------------------------
/// If needed, extend this to allow translation from
/// (TMsgIn, TErrIn) -> (TMsgOut, TErrOut)
///
/// Any TErr used should be convertible to AgentError
pub struct MsgTransformWrapperNode<F, TMsg, TErr>
{
    // Pregel node fields
    pub name: String,
    pub description: String,

    // My Fields
    wrapped_node : Box<dyn PregelNode<Error = TErr, Message = TMsg> + Send + Sync>,
    xform_func   : F,

    // To keep the compiler happy.
    msg_type       : PhantomData<TMsg>,
    err_type       : PhantomData<TErr>
}

// --------------------------------------------
impl<F, TMsg, TErr> MsgTransformWrapperNode<F, TMsg, TErr> 
{
    pub fn new(name: &str, desc: &str,
               wrap_node : Box<dyn PregelNode<Error = TErr, Message = TMsg> + Send + Sync>,
               xform     : F) -> Self 
    {
        MsgTransformWrapperNode {
            name: name.to_string(),
            description: desc.to_string(),

            wrapped_node : wrap_node,
            xform_func   : xform,

            msg_type       : PhantomData,
            err_type       : PhantomData
        }
    }
}    

// Replace with a derive-macro ----------------
#[async_trait]
impl<F, TMsg, TErr> PregelNode for MsgTransformWrapperNode<F, TMsg, TErr> 
where
    F    : Fn(TMsg) -> std::result::Result<TMsg, TErr> + Sync,
    TMsg: Send + Sync + Clone + std::fmt::Debug + 'static,
    TErr: Send + Sync + std::fmt::Debug + From<AgenticError> + 'static,
{
    type Error   = TErr;
    type Message = TMsg;

    fn name(&self) -> &String {
        &self.name
    }

    fn description(&self) -> &String {
        &self.description
    }

    async fn exec(
        &self,
        ctx: Arc<Mutex<PregelContext>>,
        in_msgs: Option<Vec<Self::Message>>,
    ) -> Result<Self::Message, Self::Error> {
        debug!("Exec called on MsgTransformWrapperNode({:?})", self.name);

        self.wrapped_node
            .exec(ctx, in_msgs)
            .await
            .map_err(|err| AgenticError::NestedNodeExecFailed(
                format!("Inner Node:{:?} errored during exec [{:?}]", self.wrapped_node.name(), err)
            ).into())
            .inspect(|inner_msg| debug!("Inner Node suceeded: Transforming it's output Msg: {:?}", inner_msg))
            .and_then(|inner_msg| (self.xform_func)(inner_msg))
            .inspect(|xformed_msg| debug!("Transformed to Msg: {:?}", xformed_msg))
    }
}
