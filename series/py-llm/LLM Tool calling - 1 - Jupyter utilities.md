# Developing LLM Tool-calling - 1 - Code Structure and Jupyter Utilities

This is the first of my short series about developing LLM tool-calling in Python. This article focuses on some intermediate setups (logging, visual decluttering, and Colab runnability) that allow a strong foundation to build better traceability, clearer output, and a portfolio that can be exposed to prospective employers via runnable notebooks. The one redeeming reference to LLMs in this article demonstrates the utils applied to OpenAI text generation.

![Series Roadmap](./img/series-progress-1.png)

## TL;DR

While the series is on tool-calling in LLMs, this article is about code organization and utilities which will be used in that effort. The goals are:
   - Minimize code repetition across notebooks _(DRY)_ by breaking code out into Python library modules.
   - Use Jupyter's builtin Markdown and HTML display for visual focus and readability. Helps tremendously when we have a lot of text output often mixed in with obscuring log statements.
   - Get into the practice of using log statements in our Python code from the start. Without this, we'll be sprinkling print statements during testing only to comment them out when done. With logs, we just use log levels: `logging.DEBUG` during development and `logging.WARNING` otherwise.
   - Learn how to run our GitHub Jupyter notebooks directly in Colab
     - Source from GitHub and launch in Colab
     - Accessing Colab secrets _(API keys)_ will be described in the next article.
   - Code Organization     
     - Each Medium article's code is on a branch off main.
     - main is the trunk to which all the individual articles/branches get merged back. 
     - Each branch serves as home for the notebooks and Python modules created per Medium article.

**Status at the end of this article:** all code under branch `main/jupyter_utils`
   - `nbs/Py_mod_jupyter_utils_devel.ipynb` where some exploration of HTML/Markdown display is performed.   
   - `nbs/Py_mod_jupyter_utils_logging_devel.ipynb` where basic logging code is developed. 
     - 👉 There is also exploration of controlling and exposing logging in third-party (OpenAI) modules
     - Since the logging output is verbose and obscures the main output, we make use of coloring code developed earlier to make our output stand out.
   - `lib/utils/jupyter_utils.py` module created from code in both the notebooks for use as a library in subsequent notebooks.          
   - `main/jupyter_utils` branch merged to main

👉 **Colab runnable Jupyter notebooks**: The links below will launch the notebooks directly in your Colab environment _(The last section in this article describes how to transform GitHub notebook URLs to Colab accessible ones)_.
   - 🔗 https://colab.research.google.com/github/vamsi-juvvi/py-llm/blob/jupyter_utils/nbs/Py_mod_jupyter_utils_devel.ipynb
   - 🔗 https://colab.research.google.com/github/vamsi-juvvi/py-llm/blob/jupyter_utils/nbs/Py_mod_jupyter_utils_logging_devel.ipynb

Continue reading for additional details.

## Python - as a Second Language

Python is pretty much the default programming language when you want to get started with LLMs _(and NLP, and Data Science and...)_ and with the rapid adoption of `type specifications`, `type-checking linters` and `async/await`, it has solidly graduated from a prototyping language to a competent backend production language. 

AI literacy is table stakes these days. Whatever your main programming language, getting comfortable with Python is very important, maybe even key to your continued career survival.

If Python is a second language _(like it is for me)_, you'll likely run into these issues each time you revisit your Python codebase:
 - Short-term memory needs to be refreshed:
   - syntax
   - tooling
   - libraries
 - When revisiting low-level libraries with WIP, entire logic flow needs to be pulled into short-term memory before making progress.

Given the above context, the process that I am describing here has worked well:
 - Build documentation that covers the head-scratchers _(VSCode and Markdown, Mermaid and the PlantUML extension)_. If you invest in learning [PlantUML syntax](http://www.plantuml.com), it becomes very fast to generate and update mind-maps or sequence diagrams directly within your VSCode Markdown. Future articles will show you how.
 - Retain Jupyter experiments as a reference. It will be cluttered, but when you need it later, invaluable.
 - Refined code from Jupyter experiments gets copied to a Python lib and reused in other notebooks.

## Jupyter in VSCode as the IDE 

There are likely many choices for a Python IDE: mine is `VSCode`. `VSCode` has full support for Python and native support for Jupyter notebooks _(it will prompt you to install some Python modules if needed)_. The excellent support for Markdown, PlantUML _(via extension)_ and Mermaid means that your documentation can live alongside your code and be edited in the same IDE. Absolutely fantastic! When exploring LLMs and GenAI, the fact that a Jupyter output cell can display syntax-colored JSON, generated images, SVG, Markdown etc. is super powerful!

> If you don't care for `VSCode` you can run Jupyter directly from your Python distribution. For the longest time, this was the main way of using Jupyter. It is still the gold standard. If you notice strange errors in VSCode's Jupyter support, maybe fall back on standalone Jupyter.

With Python as a second language and Jupyter as a second IDE:
 - Jupyter adds a layer to the Python constellation you need to remember. Especially cell magics.
 - If your custom code is not published as a Python module, then you'll end up copy/pasting it at the top of each of your notebooks. Serious violation of the `DRY` principle _(totally fine if it is a couple of throwaway notebooks. We are practical 80/20 people after all)_.
 - When there are a ton of notebooks however, as a seasoned developer, seeing duplicated code kills something in you! `DRY`: this is the way! `CRY` _(constantly repeating yourself. You heard it here first)_ is, like it says, tearful! You will cry less after these articles!

### Steps Toward Advanced Jupyter Usage

My first exposure to serious notebook-based development was with the [practical deep learning for coders](https://course.fast.ai/) course from [fast.ai](https://www.fast.ai/). If you wish to take things to the next level _(Jupyter notebooks as the source of truth)_, these resources are an excellent start.

 - [fast.ai - Notebook best practices](https://nbdev.fast.ai/tutorials/best_practices.html)
 - [End to end video of the process by J. Howard and Hamel](https://www.youtube.com/watch?v=l7zS8Ld4_iA&t=2s)

A less heavy approach to using jupyter-notebook-as-the-module, instead of copying code into a separate Python module:
 - [Importing Jupyter Notebooks as modules](https://jupyter-notebook.readthedocs.io/en/stable/examples/Notebook/Importing%20Notebooks.html).

# Jupyter Utils

- Design thoughts in the 🔗 [development notebook](https://github.com/vamsi-juvvi/py-llm/blob/jupyter_utils/nbs/Py_mod_jupyter_utils_devel.ipynb)
- 🔗 [Code](https://github.com/vamsi-juvvi/py-llm/blob/jupyter_utils/lib/util/jupyter_util.py)

## Output rendered as HTML

HTML, CSS etc. are part of a vast field. I am only providing a sample for an optionally titled colored box. Small but immensely useful to make something stand out in a sea of text.

### Library Code

The following library code in [lib/utils/jupyter_utils.py](https://github.com/vamsi-juvvi/py-llm/blob/jupyter_utils/lib/util/jupyter_util.py) handles the HTML portion.

```python
from IPython.display import display, HTML
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
```

### Sample Usage and Output

![utils colorbox sample](../img/jupyter_utils_colorbox_sample.png)

## Output as Markdown

> 🔗 [Markdown reference](https://commonmark.org/help/)

Jupyter's built-in Markdown support is excellent. I have provided some basic wrappers for the formatting I seem to use the most:
 - Heading levels
 - Tables
 - Code blocks _(especially JSON)_

### Library Code

The following library code in [lib/utils/jupyter_utils.py](https://github.com/vamsi-juvvi/py-llm/blob/jupyter_utils/lib/util/jupyter_util.py) handles the Markdown portion

```python
from IPython.display import display, Markdown

class DisplayMarkdown:
     @staticmethod
     def h(title:str, title_level:int|None=None):
          """
          Heading. 
          `title_level` defaults to 1
          """
          title_level = title_level if title_level else 1
          title_hashtrain = eval(f"\"#\"*{title_level}") if title_level > 1 else "#"
          DisplayMarkdown.md(f"{title_hashtrain} {title}")

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
```

### Sample Usage and Output

![Markdown module sample output](../img/jupyter_utils_md_sample.png)

## Additional Formatting Options

Beyond `HTML` and Markdown, the display module has support for plenty more. I have used `LaTeX` and `Math` in the past. Explore and expand the module as you need.

# Logging

Having your code print out log/traces is a great discipline to get into. There are always some considerations to keep in mind though. I will provide a brief overview. There is a ton of info out there if you want to explore further.

 - 🔗 [Python logging levels](https://docs.python.org/3/library/logging.html#logging-levels)

![Python logging mind map](../img/jupyter_utils_logging_mmap.svg)

## Basic Logging Output in Jupyter

 - uses the `setup_logging()` method defined in [lib/utils/jupyter_utils.py - TODO](TODO)
 - Standard Jupyter environment in VSCode
 - 👉 Try it out in 🔗 [Colab - GitHub link](https://colab.research.google.com/github/vamsi-juvvi/py-llm/blob/jupyter_utils/nbs/Py_mod_jupyter_utils_logging_devel.ipynb)

 ```python
 setup_logging(logging.DEBUG)

def my_func():
    # do stuff and sprinkle this in as needed
    logging.debug("My debug statement")
    logging.warning("My warning")
    logging.error("My error")

my_func()
```
↓

```log
04:23:07 DEBUG:My debug statement
04:23:07 WARNING:My warning
04:23:07 ERROR:My error
```

## Gotchas with Module Level LogConfig

👉 One very important thing to remember is that the Python logging module is configured just once at the module level _(singleton)_. Changing `os.environ["OPENAI_LOG"]="debug"` to `os.environ["OPENAI_LOG"]="error"` will have no effect if you have already run the cell!

The first run will do an equivalent of `logging.logConfig(..)` and every subsequent call to `logging.logConfig(..)` directly or indirectly, will be a `no-op`. You will pull your hair out if you are not aware of this!

**The simplest solution:**
  - Locate the `Restart Kernel` button for the notebook and restart. _This will clear out all local variables. If those need to be preserved, see the `Slightly harder solution` section_.
  - Run the cell that inits to the correct log level 👉 FIRST
  - Then run the rest. If needed, split the log init into a separate cell so you can run it with no other side effects.

**Slightly harder solution:**
  - Run `reload(logging)` to reset the module state. 
  - Stick this in a separate cell if you want.
  - In the case of OpenAI, this doesn't work, it likely means that they are caching the `OPENAI_LOG` var. `reload(openai)` doesn't work, which means the state is changed in some other module. Ultimately, I am forced to restart the kernel 😟 when I want the `OPENAI_LOG` changed.

## Debug Log OpenAI Completion 

Since this is an LLM-focused set of articles after all, I should have at least one LLM reference:

 - I have my `OPENAI_API_KEY` set in the env. The next article will show you how to save your API key in Colab and run the notebook from there. If you already have a Linux/WSL environment to clone the repo into, simply set the `OPENAI_API_KEY` before you start the jupyter notebook.

```python
import os
os.environ["OPENAI_LOG"]="error"

import openai

# Expects a OPENAI_API_KEY env var
def get_completion(prompt, model="gpt-4o-mini", temperature=0) -> str:
    chat_history = [{"role":"user", "content":prompt}]
    response = openai.chat.completions.create(
        model=model,
        messages=chat_history,
        temperature=temperature)
    return response.choices[0].message.content

print(get_completion("Why is the sky blue"))
```

### With No Logging (or Log-level Set to Error)

↪
```
The sky appears blue primarily due to a phenomenon called Rayleigh scattering. When sunlight enters the Earth's atmosphere, it is made up of different colors, each with varying wavelengths. Blue light has a shorter wavelength compared to other colors like red or yellow.

As sunlight passes through the atmosphere, it collides with gas molecules and small particles. Because blue light is scattered in all directions more than other colors due to its shorter wavelength, we see a predominance of blue when we look up at the sky.

During sunrise and sunset, the sky can appear red or orange because the sunlight has to pass through a greater thickness of the atmosphere. This longer path scatters the shorter blue wavelengths out of our line of sight, allowing the longer red wavelengths to dominate the view.
```

### With Debug Logs

```diff
- os.environ["OPENAI_LOG"]="error"
+ os.environ["OPENAI_LOG"]="debug"
```

↪
```
[2025-06-01 12:35:18 - openai._base_client:482 - DEBUG] Request options: {'method': 'post', 'url': '/chat/completions', 'files': None, 'idempotency_key': 'stainless-python-retry-303e8deb-ca6f-404d-b56b-fd3fbbb6c252', 'json_data': {'messages': [{'role': 'user', 'content': 'Why is the sky blue'}], 'model': 'gpt-4o-mini', 'temperature': 0}}
[2025-06-01 12:35:18 - openai._base_client:965 - DEBUG] Sending HTTP Request: POST https://api.openai.com/v1/chat/completions
[2025-06-01 12:35:22 - httpx:1025 - INFO] HTTP Request: POST https://api.openai.com/v1/chat/completions "HTTP/1.1 200 OK"
[2025-06-01 12:35:22 - openai._base_client:1003 - DEBUG] HTTP Response: POST https://api.openai.com/v1/chat/completions "200 OK" Headers([('date', 'Sun, 01 Jun 2025 19:35:22 GMT'), ('content-type', 'application/json'), ('transfer-encoding', 'chunked'), ('connection', 'keep-alive'), ('access-control-expose-headers', 'X-Request-ID'), ('openai-organization', 'user-uxl7oko9mdo17utucmetfrwn'), ('openai-processing-ms', '2661'), ('openai-version', '2020-10-01'), ('x-envoy-upstream-service-time', '2666'), ('x-ratelimit-limit-requests', '10000'), ('x-ratelimit-limit-tokens', '200000'), ('x-ratelimit-remaining-requests', '9999'), ('x-ratelimit-remaining-tokens', '199993'), ('x-ratelimit-reset-requests', '8.64s'), ('x-ratelimit-reset-tokens', '2ms'), ('x-request-id', 'req_73990dd32d323049fc8dfd8898d2821a'), ('strict-transport-security', 'max-age=31536000; includeSubDomains; preload'), ('cf-cache-status', 'DYNAMIC'), ('set-cookie', '__cf_bm=a4BlQQOw7l1MuEXJTJRwA2nzAvrd710qBXEUQCOnQ2Y-1748806522-1.0.1.1-oAcW4TUEp5KUQuHlWCTWatdBrwTcXaBpUUMj7qQh3MLUupttx2f3AbzSzqOUcjLYteQKBstZ_mB64LeQoS4d0Tb44Dt0SoTRdiMBCooskvQ; path=/; expires=Sun, 01-Jun-25 20:05:22 GMT; domain=.api.openai.com; HttpOnly; Secure; SameSite=None'), ('x-content-type-options', 'nosniff'), ('set-cookie', '_cfuvid=Z46zrs1L8szME9iGFG_TMhRakaK_t1ULPb6AXCTk6qY-1748806522351-0.0.1.1-604800000; path=/; domain=.api.openai.com; HttpOnly; Secure; SameSite=None'), ('server', 'cloudflare'), ('cf-ray', '94912749492b2b4d-LAX'), ('content-encoding', 'gzip'), ('alt-svc', 'h3=":443"; ma=86400')])
[2025-06-01 12:35:22 - openai._base_client:1011 - DEBUG] request_id: req_73990dd32d323049fc8dfd8898d2821a
The sky appears blue primarily due to a phenomenon called Rayleigh scattering. When sunlight enters the Earth's atmosphere, it is made up of different colors, each with varying wavelengths. Blue light has a shorter wavelength compared to other colors like red or yellow.

As sunlight passes through the atmosphere, it collides with gas molecules and small particles. Because blue light is scattered in all directions more than other colors due to its shorter wavelength, we see a predominance of blue when we look up at the sky.

During sunrise and sunset, the sky can appear red or orange because the sunlight has to pass through a greater thickness of the atmosphere. This longer path scatters the shorter blue wavelengths out of our line of sight, allowing the longer red wavelengths to dominate.
```

Awful presentation! However, 
 - We see the additional default args in the completions JSON packet
 - The HTTP-level interactions are exposed
   - We see the endpoints
   - We get a peek into the headers
     - `x-ratelimit-reset-tokens`
     - `x-ratelimit-limit-tokens`
     - `x-ratelimit-limit-requests`

Unfortunately, while very informative, the actual completion text is obscured. Thankfully, we can improve this significantly with minor tweaks.

### With a Separator - Markdown Skills FTW

The easiest way to separate the actual payload from the log output is to, well, use a separator. You can use a Markdown title as a separator if you wish. Anything that visually separates.

```diff
+ from IPython.display import display, Markdown
+ display(Markdown("----"))
print(get_completion("Why is the sky blue"))
```

or

```diff
+ from IPython.display import display, Markdown
+ m3_title = "OpenAI Response"
+ txt_completion = get_completion("Why is the sky blue")
+ display(Markdown(f"----\n### {m3_title}\n{txt_completion}"))
- print(get_completion("Why is the sky blue"))
```

### With Coloring - Take the Crayons Out

Here, we mix in the HTML output functionality and you see how those tiny utils can have outsized power!

```python
# For displaying HTML and Markdown responses from ChatGPT
from IPython.display import display, HTML

# Enhance with more Html (fg-color, font, etc) as needed but title is usually a good starting point.
def colorBox(txt, title=None):
    if title is not None:
        txt = f"<b>{title}</b><br><hr><br>{txt}"

    display(HTML(f"<div style='border-radius:15px;padding:15px;background-color:pink;color:black;'>{txt}</div>"))
```

```python
# Run to completion so all logs are printed out
res = get_completion("Why is the sky blue")

# print your result
colorBox(res, title="OpenAI Response")
```

↪

![OpenAI Completion in ColorBox](../img/oai_completion_colorbox.png)

So much improvement! The entire HTML world is open to you, but just this simple coloring is effective enough. You can enhance the color-box function to take in other colors to indicate semantically meaningful states, stick emojis or icons in etc.

# Running Your Notebook in Colab

Google's [Colab](https://colab.research.google.com/) is a great way to run notebooks. In addition to being a `Jupyter` environment, it also

 - Provides access to GPUs: you can run ML code with GPU acceleration
 - Upgrades to a paid Colab subscription that gives access to better GPUs etc. 

Now, being Google:

 - You can log in with your Google account 
 - Save/access notebooks in your Google Drive
 - Have a simple way _(Python code)_ to access your entire Google Drive for data etc.

Accessing notebooks hosted on GitHub in Colab is luckily pretty straightforward.

## Running Public GitHub Repo Files on Colab

`py-llm` is a public repo. Accessing a notebook from it _(or any public repo)_ is quite simple.
  - Locate a GitHub link: https://github.com/vamsi-juvvi/py-llm/blob/jupyter_utils/nbs/Py_mod_jupyter_utils_devel.ipynb say
    - Break this down into the following components
      - `https://github.com` / `$TAIL`
    - Convert to 
      - `https://colab.research.google.com/github` / `$TAIL`
  - The `Py_mod_jupyter_utils_devel.ipynb` link transforms into the following. Click it and see.
  - ↪ 🔗 https://colab.research.google.com/github/vamsi-juvvi/py-llm/blob/jupyter_utils/nbs/Py_mod_jupyter_utils_devel.ipynb

## Running Private GitHub Repo Files on Colab

If you have code in a private repo, you need some more steps: 

 - 🔗 [GitHub official - Using GitHub with Google Colab](https://bebi103a.github.io/lessons/02/git_with_colab.html)
 Ask your favorite search engine 
 - 🔗 [Felix Muller's article on Medium](https://felixbmuller.medium.com/connect-a-private-github-repository-with-google-colab-via-a-deploy-key-cca8ad13007)
