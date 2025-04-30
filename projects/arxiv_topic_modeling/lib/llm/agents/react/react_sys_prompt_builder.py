#----------------------------------------------------------
# See development in nbs/LLM/Py_mod_llm_react_devel.ipynb
# Code copied from there and some jupyter specific output 
# formatting taken out.
#----------------------------------------------------------
import copy
from string import Template


# The structure of a typical react system prompt.
react_system_prompt_template = Template("""
                                        
$YOUR_ROLE_AS_REACT_SECTION
                                        
$REACT_TOOLS_SECTION

$REACT_LOOP_EXEMPLARS_SECTION
                                        
$REACT_LOOP_ADDITIONAL_RULES
                                        
$REACT_INTRODUCE_CONVERSATION_SECTION
""".strip())

#-----------------------------------------------------
# Dictionary containing defautl substitutions for the template 
# above.
react_sys_prompt_template_args = {}

react_sys_prompt_template_args["YOUR_ROLE_AS_REACT_SECTION"] = """
You are designed to help with a variety of tasks, from answering questions \
to providing summaries to other types of analyses.""".strip()

# + TOOLS
react_sys_prompt_template_args["REACT_TOOLS_SECTION"] = Template("""
## Tools
You have access to a wide variety of tools. You are responsible for using
the tools in any sequence you deem appropriate to complete the task at hand.
This may require breaking the task into subtasks and using different tools
to complete each subtask.

You have access to the following tools:
$TOOLS
""".strip())

# + TOOL_NAMES_CSV
# + REACT_CONCLUSION_WITH_SUCCESS_EXEMPLAR
#     Thought: I can answer without using any more tools.
#     Answer: [your answer here]
#
# + REACT_CONCLUSION_WITH_FAILURE_EXEMPLAR
#     Thought: I cannot answer the question with the provided tools.
#     Answer: Sorry, I cannot answer your query.
react_sys_prompt_template_args["REACT_LOOP_EXEMPLARS_SECTION"] = Template("""
## Output Format
To answer the question, please use the following format.

```
Thought: I need to use a tool to help me answer the question.
Action: tool name (one of $TOOL_NAMES_CSV) if using a tool.
Action Input: the input to the tool, in a JSON format representing the kwargs (e.g. {{"input": "hello world", "num_beams": 5}})
```

Please ALWAYS start with a Thought.

Please use a valid JSON format for the Action Input. Do NOT do this {{'input': 'hello world', 'num_beams': 5}}.

If this format is used, the user will respond with an observation in the following format:

```
Observation: tool response
```

You should keep repeating the above format until you have enough information
to answer the question without using any more tools. At that point, you MUST respond
in the one of the following two formats:

```
$REACT_CONCLUSION_WITH_SUCCESS_EXEMPLAR
```

```
$REACT_CONCLUSION_WITH_FAILURE_EXEMPLAR
```
                                                                          
Please Pay attention to the following instructions:
  - You MUST obey the function signature of each tool. Do NOT pass in no arguments if the function expects arguments.
""".strip())

# In the LlamaIndex example, this was done as an enhancement step when 
# refining the prompt. Retain it as the LLAMA_REACT_LOOP_ADDITIONAL_RULES 
# constant (if you want to try it out) but default it to "".
react_sys_prompt_template_args["REACT_LOOP_ADDITIONAL_RULES"] = ""

react_sys_prompt_template_args["REACT_INTRODUCE_CONVERSATION_SECTION"] = """
## Current Conversation
Below is the current conversation consisting of interleaving human and assistant messages.
""".strip()

#--------------------------------------------------------------------------
# Extra constants
#--------------------------------------------------------------------------
# Possible values to try for additional rules. Taken from the LlamaIndex 
# examples
LLAMA_REACT_LOOP_ADDITIONAL_RULES = """
## Additional Rules
- The answer MUST contain a sequence of bullet points that explain how you arrived at the answer. This can include aspects of the previous conversation history.        
""".strip()

#--------------------------------------------------------------------------
# Util methods
#--------------------------------------------------------------------------
def are_all_vars_resolved(tmpl: Template) -> bool :
        try:
            tmpl.substitute({})
            return True
        except ValueError:
            return False        

#--------------------------------------------------------------------------
# The builder itself
#--------------------------------------------------------------------------
class ReactSysPromptBuilder:    

    def __init__(self):        
        # the keys of subst_args will be resolved incrementally. So use a 
        # deep copy to leave template args intact.
        self.subst_args = copy.deepcopy(react_sys_prompt_template_args)
        self.tmpl       = react_system_prompt_template

    #-------------------------------------    
    def build_safe(self) -> str:
         """Does not fail even if variables are unresolved"""
         # low level resolve in a copy
         resolved_args = copy.deepcopy(self.subst_args)
         for k,v in resolved_args.items():
              if isinstance(v, Template):
                   resolved_args[k] = v.safe_substitute({})

         return self.tmpl.safe_substitute(resolved_args)

    #-------------------------------------
    def override_role(self, role_arg: str | None):
         """
         Call if you want a different role than the default
         ReAct role.
         """
         ROLE_SECTION_KEY = "YOUR_ROLE_AS_REACT_SECTION"
         if role_arg:
            self.subst_args[ROLE_SECTION_KEY] = role_arg        

    #-------------------------------------
    def init_tools_tmpl(self, tools_arg:str):
        TOOLS_SECTION_KEY="REACT_TOOLS_SECTION"
        TOOLS_CHILD_KEY="TOOLS"          

        # Update the EXEMPLARSSECTION_KEY template
        self._do_update_tmpl_arg(
             TOOLS_SECTION_KEY,
             {
                TOOLS_CHILD_KEY : tools_arg if tools_arg else "TOOLS is NOT SPECIFIED"
             })            
    
    #-------------------------------------
    def init_exemplars_tmpl(self, 
                            tool_names_csv : str,
                            success_example:str | None, 
                            cannot_answer_example: str | None):
        """
        Must be called to set the tool_names_csv as it has no meaningful default.
        
        The success_example and cannot_answer_examples can be overridden 
        but have default values.
        """

        EXEMPLARSSECTION_KEY="REACT_LOOP_EXEMPLARS_SECTION"

        CHILD_KEY_TOOL_NAMES_CSV = "TOOL_NAMES_CSV"
        CHILD_KEY_SUCCESS        = "REACT_CONCLUSION_WITH_SUCCESS_EXEMPLAR"
        CHILD_KEY_CANNOT_ANSWER  = "REACT_CONCLUSION_WITH_FAILURE_EXEMPLAR"

        # Defaults        
        DEFAULT_SUCCESS_EXAMPLE = """
Thought: I can answer without using any more tools.
Answer: [your answer here]
            """.strip()
             
        DEFAULT_CANNOT_ANSWER_EXAMPLE = """
Thought: I cannot answer the question with the provided tools.
Answer: Sorry, I cannot answer your query.
             """.strip()        

        # Update the EXEMPLARSSECTION_KEY template
        self._do_update_tmpl_arg(
             EXEMPLARSSECTION_KEY,
             {
                CHILD_KEY_TOOL_NAMES_CSV : tool_names_csv if tool_names_csv else "Tool names NOT SPECIFIED",
                CHILD_KEY_SUCCESS        : success_example if success_example else DEFAULT_SUCCESS_EXAMPLE,
                CHILD_KEY_CANNOT_ANSWER  : cannot_answer_example if cannot_answer_example else DEFAULT_CANNOT_ANSWER_EXAMPLE,
            })                
        
    #-------------------------------------
    def init_additional_rules_tmpl(self, 
                            additional_rules : str | None = None):
        """
        You don't always want additional rules. 
        Defaults to empty "". 

        Call with REACT_LOOP_ADDITIONAL_RULES constant if you want to try 
        the 'conclude with reasons` version that LlamaIndex tried in their refined
        version of the prompt.
        """        
        ADDITIONAL_RULES_SECTION_KEY="REACT_LOOP_ADDITIONAL_RULES"                
        self._do_update_string_arg(
            ADDITIONAL_RULES_SECTION_KEY,
            additional_rules
        )         
    
    #-------------------------------------
    def _do_update_tmpl_arg(self, key:str, subst_dict:dict):
        """
        Updates self.subst_args[key] 's template value with the supplied
        substitutions. The updated value will remain a Template when all
        values are fully resolved.
        
        This is done safely with no exceptions which means 
        the resulting template can still have unresolved variables. 
        """
        assert(isinstance( self.subst_args[key], Template))

        self.subst_args[key] = Template(
                 self.subst_args[key].safe_substitute(subst_dict)
        )
    
    def _do_update_string_arg(self, key:str, subst_str:str|None=None):
        """
        Replaces self.subst_args[key] 's string value with the supplied
        string. 

        If None is suppplied, then this is cleared out (set to "")
        """
        assert(isinstance( self.subst_args[key], str))

        self.subst_args[key] = subst_str if subst_str else ""