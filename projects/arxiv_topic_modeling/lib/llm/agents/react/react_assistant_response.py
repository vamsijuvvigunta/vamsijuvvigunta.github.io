#----------------------------------------------------------
# See development in nbs/LLM/Py_mod_llm_react_devel.ipynb
# Code copied from there and some jupyter specific output 
# formatting taken out.
#----------------------------------------------------------
import re
import logging

class ReactAssistantResponse:
    PATTERN_TH        = re.compile(r"^(Thought|Action|Action Input|Answer)\s*:\s*(.*?)$", re.MULTILINE)
    PATTERN_FUNC_NAME = re.compile(r"^\s*(?:function[s]?\.)?(.*)$")

    def __init__(self, assistant_response:str):
        self.thought      = None
        self.action       = None
        self.action_input = None
        self.answer       = None

        #-- Parse -----------------        
        match_list = self.PATTERN_TH.findall(assistant_response)

        if match_list or len(match_list) > 0:
            d = {}
            for m in match_list:
                key = m[0]
                val = m[1]
                logging.debug(f"Extracted [{key} = {val}] pair")
                d[key] = val
            self._init_kvps(d)
        else:
            logging.debug("Could not extract any exected React semantic sections from assistant response\n{assistant_response}")
    
    def __str__(self):
        return f"""
Thought     : {self.thought}
Action      : {self.action}
Action Input: {self.action_input}""".strip()

    def _init_kvps(self, d):
        for k,v in d.items():
            match k.lower():
                case "thought":
                    self.thought = v.strip()
                case "action":
                    # These seem to sometimes comes in as `function.my_action`
                    fn_match = self.PATTERN_FUNC_NAME.match(v)
                    if fn_match:
                        self.action = fn_match.group(1)
                        logging.debug(f"Got Action = {self.action}")
                    else:
                        logging.error(f"Unable to get function name from Action=\"{v}\"")
                case "action input":
                    self.action_input = v
                    logging.debug(f"Got Action Input = {v}")
                case "answer":
                    self.answer = v
                    logging.debug(f"Got Answer = {v}")
                case _:
                    logging.warning(f"Unknown Key={k} with Value={v}")