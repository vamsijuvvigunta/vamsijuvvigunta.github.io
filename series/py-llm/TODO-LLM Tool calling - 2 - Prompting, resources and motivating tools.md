# LLM Tool calling - 2 - Prompting, resources and motivating tools

This is part of my short series on developing LLM tool-calling and usage in Python. The previous article is [LINK NEEDED - LLM Tool calling - 2 - Colab, Imports and OpenAI Utils](..).

I thought I could get on to introducing tools but there is a large gap between a simple `why is the sky blue` and the introduction of tools. I will detail some of my notes on prompting here and some additional resources: there is a huge body of research exploring and pushing the frontier on what you can get an LLM to do.

LLM and prompt engineer is a very approachable subject area _(the various ArXiv papers are also very readable)_. It is quite easy to get started with prompting an LLM. Early 2022 saw me building the solutioning vocabulary for a `test` → `animation` problem: I started with NLP concepts like `NER`, `SRL`, `UDep` and `CoRef` but eventually overcame the sunk-cost inertia and started looking at LLMs. I wanted to figure out how I could retool my solutioning around `LLMs`. No clear was to be had! it seemed that if you could get a prompt to do all of what you want, great! Otherwise, you had to explore an engineering solution around the limits of the LLMs. So you have to then explore the LLM prompting space to figure out what the limitations are. 

Even as late as 2024, research is ongoing to explore the frontiers of what you get a single LLM to do. See these gems for instance
 - [Arxiv - Solo Performance Prompting](https://aclanthology.org/2024.naacl-long.15.pdf)
 - [Prompt to creating teaching blueprint prompts](https://hbsp.harvard.edu/inspiring-minds/an-ai-prompting-template-for-teaching-tasks/) Scroll down to the third page to see the absolute unit of a prompt in use. A prompt that can generate a customized prompt for a particular subject area and then help the lecturer craft study materials. **This prompt generates a blueprint which is itself a prompt**. The authors supply the custom GPT for this at [Teaching-assistant-blueprint-maker](https://chatgpt.com/g/g-ck4wESI9U-teaching-assistant-blueprint-maker).
 
## Resources

There are simply too many to list. I am listing some of the meta ones.

 - 

## Best practices

