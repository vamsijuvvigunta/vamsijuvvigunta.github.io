# LLM Tool-calling - 4 - Developing the ReAct loop

This is the fourth article in my series about developing tool-calling in Python. We have developed basic tool-calling in [LINK - Previous medium article](..). This articles builds on tool-calling and implements a higher-level abstraction layered on top.

## Series roadmap

![](./img/series-4/series-4-react.png)

## TL;DR

 - The ReAct prompting protocol that employs COT to generate a (`thought` → `action` → `observation` ↩ ) loop on the path to generating an answer.
 - Enhance `Tool` and `ToolCollection` classes to generate suitably condensed versions of a tool-spec that can be included inside a prompt.
 - Build a `react_chat_loop` that maps the chat protocol (`user`, `assistant`, `tool_call`, `tool` into the (`thought` → `action` → `observation` ↩→ `answer` ) protocol
 - Exercise a multi-step arithmetic example.
 - Development notebooks: [Py_mod_llm_agentic_react_devel.ipynb](https://github.com/vamsi-juvvi/py-llm/blob/4_tools_react/nbs/Py_mod_llm_agentic_react_devel.ipynb) and [Py_mod_llm_controller_tools_devel.ipynb](https://github.com/vamsi-juvvi/py-llm/blob/3_llm_tools_and_support/nbs/Py_mod_llm_controller_tools_devel.ipynb)
 - Colab runnable notebooks: [Colab - Py_mod_llm_agentic_react_devel.ipynb](https://colab.research.google.com/github/vamsi-juvvi/py-llm/blob/4_tools_react/nbs/Py_mod_llm_agentic_react_devel.ipynb)
 - Python module code at [py_llm/llm/tools.py](https://github.com/vamsi-juvvi/py-llm/blob/4_tools_react/lib/py_llm/llm/tools.py) and [py_llm/llm/agentic/react/](https://github.com/vamsi-juvvi/py-llm/blob/4_tools_react/lib/py_llm/llm/agentic/react/)


Read on for more details.

## ReAct

ReAct is one of the earliest _thinking_ prompting techniques where the LLM was asked to breakdown it's task into multiple turns and incorporate external tools in it's process before responding. The hope _(and finding)_ is that it would move away from simple recall to a more complex chain of thought _(including backtracking)_ yielding more appropriate answers.

The most basic `ReAct` form is the one mentioned in the paper that introduced it : [ReAct: Synergizing Reasoning and Acting in Language Models](https://arxiv.org/pdf/2210.03629). See also the companion [react-lm.github.io](https://react-lm.github.io/) site.


### COT and ECHO
 
 **Papers**
 - [📃 arXiv - Chain-of-Thought prompting elicits reasoning in large language models - Jason Wei et. al.](https://arxiv.org/pdf/2201.11903)   
 - [📃 arXiv - Self Harmonized Chain of Thought - Ziqi Jin, Wei Lu](https://arxiv.org/pdf/2409.04057)

**Prompting COT**
 - [Learnprompting.org - COT](https://learnprompting.org/docs/intermediate/chain_of_thought) 

 `COT` pioneered the approach of prompting an LLM to `think` in intermediate steps rather them simply recall _(or hallucinate)_ a response. The `think` ability is wishful, anything that seems to involve a plan is considered thinking. Once we craft a custom chain-of-though _(prompt engineer earning his/her pay)_ we can reliably expect a quality LLM to mimic it. It will break down our ask in a similar fashion and then generate it's own chains of thought to solve the problem.

 Later, folks discovered [zero-shot COT](https://learnprompting.org/docs/intermediate/zero_shot_cot) where you don't even have to craft a custom `COT`. One simply tells the LLm to **think step by step**. 

## ReAct and COT

[ReAct: Synergizing Reasoning and Acting in Language Models](https://arxiv.org/pdf/2210.03629) was created after LLMs got `tool` support. ReAct, in short is a 
 - Generic COT instructions to follow a `thought`, `action`, `observation` loop where
   - `actions` were mapped to `tools`
   - `observations` were the result of making the tool call. One of:
     - action's positive result
     - action's error result
     - unknown tool, please limit yourself to the provided actions
 - The power of this technique derives from the LLM's language understanding. Just like in tool-calling, the list of available actions, tells the LLM what options it has and it calls the most appropriate one and observes. Unlike tool-calling though, this can be done in a loop and the **LLM implements some sort of back-tracking to try the next best action if the earlier choices failed**.

### ReAct overlayed on the LLM Chat protocol.

As seen in the tool-calling, the completion end-points implement a multi-turn protocol involving three personas/roles (`user`, `system`, `assistant`) and two specialized rules for tool support (`tool_call`, `tool`).

ReAct is layered on top of this chat protocol. While the implementation could use tools, it seeks to overlay it's protocol on the traditional (`user`, `system`, `assistant`) roles and does not utilize actual tool support (`tool_call`, `tool`). **The implementation however** can freely implement `actions` as `tools`.

![](./img/series-3/tool_calling_protocol.png)
↪
![](./img/series-4/react_chat_mm.png)
⇕
![](./img/series-4/react_highlevel_protocol.png) 


## Enhancing the code

TBC..