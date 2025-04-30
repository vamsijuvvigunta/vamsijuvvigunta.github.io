import logging
import llm.openai_util as oai
from   llm.tools import ToolCollection

def run_chat_loop(prompt:str, tools : ToolCollection | None):
    """
    Runs a chat loop with an initial prompt and supplied tools
    Resolves all tool_calls made till a final assistant response is provided

    If a tool_call is made by the LLM and no tools are supplied, a ValueError is raised.
    """
    # Initialize
    chat_history = [
        {
            "role" : "system",
            "content" : "You are a helpful assistant that uses the supplied tools to respond to the user's questions."
        }]

    tool_schemas = tools.get_schemas() if tools else []

    # Run the loop
    msgs = [{
        "role":"user", 
        "content": prompt}]
    
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

            # The tool-call set needs to be added back to the chat_history
            msgs.append(response.choices[0].message)

            # Process all the tool calls
            for tool_call in response.choices[0].message.tool_calls:
                logging.debug(f"Requested tool_call: {tool_call}")
                logging.debug(f"Attempting to call {tool_call.function.name}({tool_call.function.arguments})")
                if tools:
                    tool_result = tools.exec_tool(
                        tool_call.function.name,
                        tool_call.function.arguments)
                    assert(isinstance(tool_result, str))
                else:
                    logging.error(f"Received tool call for {tool_call.function.name} but ToolColletion is None")
                    raise ValueError("Got tool-call request from LLM but ToolCollection is empty")

                # along with it's response. The response will be linked to the tool_call's 
                # via the ID        
                msgs.append({
                    "role" : "tool",
                    "tool_call_id" : tool_call.id,
                    "content"      : tool_result
                })
        else:
            # Assistant response
            chat_response = response.choices[0].message.content
            logging.info(f"Received final response : {chat_response}")

            # Add response to chat history and leave msgs empty
            # this will terminate the chat
            chat_history.append({
                "role" : "assistant",
                "content" : chat_response
            })
    
    # return final chat_history item as the response.
    # assert that it is from assistant ?
    return chat_history[-1]["content"]