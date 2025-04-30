# Utility methods to visually separate output from logs
# displaying HTML and Markdown responses
import os
import logging
import json
from IPython.display import display, HTML, Markdown

#----------------------------------------------------------
#  Include something like this before your python cells
#----------------------------------------------------------
#  # Setup paths to our libs
#  import sys
#  from pathlib import Path
#  
#  sys.path.append(str(Path("../../libs/python/hillops").resolve()))
#  
#  # Import jupyter utils
#  import logging
#  from util import jupyter_util
#  from util.jupyter_util import DisplayHTML as jh
#  from util.jupyter_util import DisplayMarkdown as jm
#  
#  # Init jupyter env
#  jupyter_util.setup_logging(logging.WARN)
#  jupyter_util.ColabEnv.import_api_keys()
#----------------------------------------------------------
def setup_logging(level = logging.DEBUG):
     """
     Supply one of logging.INFO|DEBUG|WARN|ERROR
     """
     from importlib import reload
     import logging
     reload(logging)
     logging.basicConfig(format='%(asctime)s %(levelname)s:%(message)s', 
                    level=level, 
                    datefmt='%I:%M:%S')


#--------------------------------------------------------------
class ColabEnv:
    """
    Expected to be run in a Jupyter environment (normal or Colab)
    """
    @staticmethod
    def import_api_keys():
        """
        Imports the various API keys from cloab's userdata.

        These are stored in colab secrets and you'll need to allow the notebook to access
        the keys explicitly on a key-by-key basis.
        """
        if 'google.colab' in str(get_ipython()):
            from google.colab import userdata
            logging.debug("Tryign to fetch OPENAI_API_KEY from your secrets. Remember to make it available to this notebook")
            os.environ["OPENAI_API_KEY"] = userdata.get("OPENAI_API_KEY")

#--------------------------------------------------------------
class DisplayHTML:
     """
     Collection of jupyter visualization methods
     """
     @staticmethod
     # Enhance with more Html (fg-color, font, etc) as needed but title is usually a good starting point.
     def color_box(txt, title=None):
          if title is not None:
               txt = f"<b>{title}</b><br><hr><br>{txt}"

          display(HTML(f"<div style='border-radius:15px;padding:15px;background-color:pink;color:black;'>{txt}</div>"))
     
     def text(txt,bg:str=None, fg:str=None):
         bg = f"background-color:{bg}" if bg else ''
         bg = f"color:{fg}" if fg else ''
         display(HTML(f"<span style='{bg};{fg};'>{txt}</span>"))


#--------------------------------------------------------------
class DisplayMarkdown:
     @staticmethod
     def h(heading:str, level:int|None=None):
          """
          Heading. 
          `level` defaults to 1
          """
          level = level if level else 1
          hashtrain = eval(f"\"#\"*{level}") if level > 1 else "#"
          DisplayMarkdown.md(f"{hashtrain} {heading}")

     @staticmethod
     def json(jsn, indent=4):
          """
          Display JSON.
          If String      → displayed as is
          If Json object → Displayed with the optional indent
                           Optional indent defaults to 4
          """          
          DisplayMarkdown.md(
               DisplayMarkdown.json_fmt(jsn, indent)
          )
     
     @staticmethod
     def hr():
          """
          like the <hr> of HTML
          Draws a separator using the "----" Md
          """
          DisplayMarkdown.md("----")

     @staticmethod            
     def code(code_block_str:str, code_lang:str|None=None):
          """
          Display Markdown code
          code_lang defaults to empty which will simply produce a ```...``` block
          """          
          DisplayMarkdown.md(
               DisplayMarkdown.code_fmt(code_block_str, code_lang)
          )

     @staticmethod
     def md(markdown_text:str):
          """
          Display markdown formatted text
          """
          display(Markdown(markdown_text))

     @staticmethod
     def code_fmt(code_block:str, code_lang:str|None=""):
          return f"```{code_lang}\n{code_block}\n```"
     
     @staticmethod
     def json_fmt(jsn, indent=4):
          if not isinstance(jsn, str):               
               jsn = json.dumps(jsn, indent=indent)               

          return DisplayMarkdown.code_fmt(jsn, "json")
     