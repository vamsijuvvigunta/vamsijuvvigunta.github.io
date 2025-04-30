#----------------------------------------------------------
# See development in nbs/LLM/Py_mod_llm_react_devel.ipynb
# Code copied from there and some jupyter specific output 
# formatting taken out.
#----------------------------------------------------------
import logging
import llm.openai_util as oai
from   llm.tools import ToolCollection
from   react_assistant_response import ReactAssistantResponse
from   react_observation import react_observation_from_action

def run_react_loop(sys_prompt: str, start_prompt:str, tools : ToolCollection | None):
    """
    Runs a chat loop with an initial prompt and supplied tools
    Resolves all tool_calls made till a final assistant response is provided

    If a tool_call is made by the LLM and no tools are supplied, a ValueError is raised.
    """
    if not sys_prompt  : raise ValueError("run_react_loop: `sys_prompt` must be supplied")
    if not start_prompt: raise ValueError("run_react_loop: `start_prompt` must be supplied")

    # Initialize
    chat_history = [ { "role" : "system", "content" : sys_prompt}]    

    # Not sure if we should be supplying tools via `tools=` or only 
    # executing the ones that are embedded in the sys_prompt ?
    # For now, null this out.
    tool_schemas = [] # tools.get_schemas() if tools else []

    # Run the loop
    # The msgs list also controls loop continutation. When msgs is empty, 
    # the loop ends
    msgs = [{
        "role":"user", 
        "content": start_prompt}]
    
    while len(msgs):
        chat_history.extend(msgs)
        msgs = []

        response = oai.get_response(
            chat_history=chat_history,
            tools = tool_schemas)

        # tool-call
        # Note: The OpenAI example is outdated
        # tool_calls is not longer a JSON object but an array of 
        # `ChatCompletionMessageToolCall` objects
        if response.choices[0].message.tool_calls:

            # We do not expect actual tool_calls
            # These comes in indirectly via an assistant response that 
            # asks for an 'Action`.
            # We could later extend this into either
            #   - An observation that it is making a tool-call instead of 
            #     sending an action. Essentially turn this exception into 
            #     an Observation.
            raise NotImplementedError("Got tool-call in react-loop. Not implemented")            
        else:
            # Assistant response
            chat_response = response.choices[0].message.content            
            logging.info(f"Received assistant response : {chat_response}")

            # Add response to chat response
            chat_history.append({
                "role" : "assistant",
                "content" : chat_response
            })

            # Followup on the assistant response
            parsed_response = ReactAssistantResponse(chat_response)
            print(str(parsed_response))

            match react_observation_from_action(parsed_response, tools):
                case None:
                    # Nothing added to msgs.
                    # loop will end.
                    logging.debug("🛑 React loop is terminated")
                case o:                    
                    logging.info(o)
                    msgs.append({
                        "role" : "user",
                        "content": o
                    })

    
    # return final chat_history item as the response.
    # assert that it is from assistant ?
    return chat_history[-1]["content"]