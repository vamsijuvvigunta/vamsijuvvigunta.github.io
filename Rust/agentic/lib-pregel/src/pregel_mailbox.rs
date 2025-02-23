use std::collections::VecDeque;

//-------------------------------------------------
#[derive(Debug, Default)]
pub struct PregelMailbox<TMsg: Clone> {
    pub inbox  : VecDeque<TMsg>,
    pub outbox : VecDeque<TMsg>
}

impl<TMsg> PregelMailbox<TMsg>
    where TMsg: Clone 
{
    pub fn new () -> PregelMailbox<TMsg> {
        PregelMailbox {
            inbox : VecDeque::<TMsg>::new(),
            outbox: VecDeque::<TMsg>::new(),
        }
    }


}