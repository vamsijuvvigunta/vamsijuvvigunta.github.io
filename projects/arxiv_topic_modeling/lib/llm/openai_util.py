import os
import openai
import logging
from typing import TypeVar

openai.api_key = os.environ.get("OPENAI_API_KEY")

#-------------------------------------------------------------------------------------
def get_completion(prompt, model="gpt-4o-mini", temperature=0) -> str:
    """
    Returns the one-shot completion of a simple prompt (no tools)
    as a string response.
    """
    chat_history = [{"role":"user", "content":prompt}]
    response = get_response(chat_history=chat_history, 
                            model=model,
                            temperature=temperature)    
    return response.choices[0].message.content

#-------------------------------------------------------------------------------------
def get_response(chat_history, model="gpt-4o-mini", tools = None, temperature=0) -> str:
    """
    Performs a single-turn with the chat-completions API given a chat history and tools
    The caller should handle tool-calls etc.
    """    
    return openai.chat.completions.create(
        model=model,
        messages=chat_history,
        tools=tools,
        temperature=temperature)
