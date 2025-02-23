//-------------------------------------------------------
pub struct MsgSource<T> {
    pub msgs: Vec<T>,
}

impl<T> MsgSource<T> {

    pub fn new() -> Self {
        MsgSource{ msgs: vec![]}
    }

    pub fn from(src: Vec<T>) -> Self {
        MsgSource{ msgs: src}
    }

    /// Left/Right Split
    /// L : The retained (left) split teed'd off an extended into `taken`. 
    ///     Note that Vec<T> implements the Extend<T> trait
    /// 
    /// T : The remaining ones retained in Self that is also 
    ///     returned to allow further chaining
    pub fn split_take<
        F : Fn(Vec<T>) -> (Vec<L> /*left-side:taken*/, Vec<T> /*right-side:remaining*/),        
        L
        >(mut self, take_into:&mut impl Extend<L>, filter_func:F) -> MsgSource<T> {
            let (take, remain) = filter_func(self.msgs.drain(..).collect());

            take_into.extend(take);

            self.msgs.extend(remain);
            self            
    }

    /// Left/Right Split
    /// L : The retained (left) split teed'd off into `taken`
    /// T : The remaining ones 
    /// 
    /// Returns (Vec<L>, Vec<T>)
    pub fn split_drain<
        F : Fn(Vec<T>) -> (Vec<L> /*left-side:taken*/, Vec<T> /*right-side:remaining*/),
        L
        >(mut self, filter_func:F) -> (Vec<L>, Vec<T>) {
            let (left, right) = filter_func(self.msgs.drain(..).collect());

            (left, right)
    }
}

// conversions from plain vectors
impl<T> From<Vec<T>> for MsgSource<T> {
    fn from(value: Vec<T>) -> Self {
        MsgSource::from(value)
    }
}

//--------------------------------------------------------

#[cfg(test)]
mod tests { 
    use genai::chat::MessageContent;
    use itertools::{Itertools, Either};
    use crate::agentic_msgs::AgenticMessages;    
    use crate::nodes::llm::{LLMPromptMsg, LLMResponseMsg};
    use crate::util::msg_filter::MsgSource;
    use crate::build_block_pattern_partition;
    use assert_matches::*;
    
    #[test]
    fn test_split_drain() {

        let msgs = MsgSource{msgs: vec![
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("Hello")),
            AgenticMessages::AMPromptResponse(LLMResponseMsg {
                prompt  : LLMPromptMsg::default_system().append_user("Hello"),
                response: "How are you!".to_string(),
            })
        ]};

        let (taken, retained) = msgs.split_drain(|mut mv| {
            build_block_pattern_partition!(
                mv,
                outer@AgenticMessages::AMPrompt(_), outer)
        });                
        
        // left should match filter
        assert_eq!(taken.len(), 1);
        assert_matches!(
            match &taken[0] {
                AgenticMessages::AMPrompt(LLMPromptMsg{chat_request:r,..}) => r.messages[0].content.clone(),
                _ => "".into()
            },
            MessageContent::Text(txt@_) if txt == "Hello");

        // right is remaining
        assert_eq!(retained.len(), 1);
        assert_eq!(
            match &retained[0] {
                AgenticMessages::AMPromptResponse(LLMResponseMsg{prompt:_, response: r}) => r.as_ref(),
                _ => ""
            },
            "How are you!");
    }    

    #[test]
    fn test_split_take_multi() {

        let msgs = MsgSource{msgs: vec![
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("Hello")),
            AgenticMessages::AMPrompt(LLMPromptMsg::default_system().append_user("World")),
            AgenticMessages::AMPromptResponse(LLMResponseMsg {
                prompt  : LLMPromptMsg::default_system().append_user("Hello"),
                response: "How are you!".to_string(),
            }),            
        ]};

        let mut prompts  =  Vec::<LLMPromptMsg>::new();
        let mut responses =  Vec::<LLMResponseMsg>::new();

        let remaining = msgs.split_take(&mut prompts,
            |mut mv| { build_block_pattern_partition!(
                    mv,
                    AgenticMessages::AMPrompt(inner @ _), inner)
            }).split_take(&mut responses, 
            |mut mv| { build_block_pattern_partition!(
                    mv,
                    AgenticMessages::AMPromptResponse(inner @ _), inner)
            });        
        
        // first split into LLMPrompt msgs
        assert_eq!(prompts.len(), 2);        

        // next split into responses
        assert_eq!(responses.len(), 1);
        
        // Fully drained
        assert_eq!(remaining.msgs.len(), 0);
    }    
}