#----------------------------------------------------------
# See development in nbs/LLM/Py_mod_llm_react_devel.ipynb
# Code copied from there and some jupyter specific output 
# formatting taken out.
#----------------------------------------------------------
import logging
from react_assistant_response import ReactAssistantResponse
from llm.tools import ToolCollection

class ReactObservation:    
    def format_observation(content:str):
        # Note that our system prompts tells the LLM to expect the Observation
        # in triple single-quotes.
        return f"""```
Observation: {content}
```
"""
    
    def from_action_response(tool_response: str):
        return ReactObservation.format_observation(
            tool_response
        )
    
    def from_action_error(ar:ReactAssistantResponse, e):
        return ReactObservation.format_observation(
            f"There was an error executing `Action: {ar.action}` with `Action Input: {ar.action_input}. "
            f"The python excepion is as follows: {str(e)}. "
            f"If possible, try again with a different action or different inputs. Remember, pay attention to JSON formatting"
        )

    def from_missing_action(assistant_response:ReactAssistantResponse, tools: ToolCollection):
        return ReactObservation.format_observation(
            f"The Action `{assistant_response.action}` is unknown. Cannot execute it. "
            f"Try one of the available ones [{", ".join(tools.get_tool_names())}]"
        )


#-------------------------------------------------------------------------------
def react_observation_from_action(ar:ReactAssistantResponse, tools : ToolCollection):
    """
    The Observation indicates continutation. If this function returns None, that means
    the react-lop has ended.
    """
    if ar.answer:
        logging.debug("Terminating ReAct loop as an answer has been provided.")
    else:
        logging.debug("Continuing react loop. Executing Action asked for.")
        assert(ar.action)

        if tools.has_tool(ar.action):
            try:
                logging.debug(f"Executing action/tool {ar.action} with args {ar.action_input}")
                action_response = tools.exec_tool(
                    name = ar.action, 
                    args = ar.action_input)
                
                return ReactObservation.from_action_response(str(action_response))
            
            except Exception as e:
                logging.error(f"Executing action raise {str(e)}")
                return ReactObservation.from_action_error(ar, e)
        else:
            logging.error(f"Assistant asked for Action:{ar.action}. There is no such tool!")
            return ReactObservation.from_missing_action(ar)