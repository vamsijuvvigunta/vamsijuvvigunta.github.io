## Regd the documents and images

 - I am a huge fan of [PlantUML](https://www.plantuml.com/) _(and mermaid for certain image types)_. With the right VSCode extension, I find it pretty fast to build these up.
 - Writing, I find, helps complete the understanding process: hence the large number of various mindmaps and sequence diagrams in all my repos.


## Py LLM series

I had done some work in 2024/2025 to buildup tool-calling functionality into the genai rust crate (and later into my own WIP agentic runtime). This was repackaged into a series of medium articles backed by a github repo. 

The initial tool support in Rust (see [PR Docs](https://github.com/vamsi-juvvi/rust-genai/tree/function_calling_openai/docs/add-function-calling)) was continued in python for jupyter notebook experimentation. I wanted to figure out how to compress the tool schema representation for ReAct use cases. See the following snippet from [examples/c06-tool-functions.rs](https://github.com/vamsi-juvvi/rust-genai/blob/function_calling_openai/examples/c06-tool-functions.rs) in my fork of the genai crate that generates a json schema given a rust function.

```rust
// Generate the schema shown in the comments above 
// - from the definition of GetCurrentWeatherParams
// - Manually add name/desc of function.
let gcw_tool_schema = schema_for_fn_single_param::<GetCurrentWeatherParams>(
    "get_current_weather".to_string(), 
    "Get the current weather".to_string(),
);
```    


While the python code was built organically, I tried to bring some discipline to the development for the medium articles.: recording all history of development, utilizing logging and adding colored outputs. The coloring utils help the cell output stand out when logging (Also great for multi-turn chats). Much simpler than dealing with gradio UIs.

 - The [medium articles](https://medium.com/@juvvij) (_#4 about ReAct loops is pending_)
 - The [py-llm git repo](https://github.com/vamsi-juvvi/py-llm) referenced in the articles
 - The markdown source for the docs 
   - [LLM Tool calling - 1 - Jupyter utilities.md](./series/py-llm/LLM%20Tool%20calling%20-%201%20-%20Jupyter%20utilities.md)
   - [LLM Tool calling - 2 - Colab, Imports and OpenAI Utils](./series/py-llm/LLM%20Tool%20calling%20-%202%20-%20Colab,%20Imports%20and%20OpenAI%20Utils.md)
   - [LLM Tool calling - 3 - Developing LLM tools](./series/py-llm/LLM%20Tool%20calling%20-%203%20-%20Developing%20LLM%20tools.md)


## Rust projects

I started working in rust only in mid 2024 _(moved on from Scala)_. Still, took to it very stongly and absolutely loving it. Got one small PR accepted, another one pending and pushed some web-server modifications (gateway/worker arch) to my own fork. I Would love an opportunity to use it in a demanding environment that will bring out the best in everyone.

### Rust - OSS contributions

  - **rust-web-app** 
    - [architecture notes on github](https://github.com/vamsi-juvvi/rust-web-app/tree/main/docs/00_base-rust-web-app)
    - Refactor rust-web-app
      - [Design/Coding docs](https://github.com/vamsi-juvvi/rust-web-app/tree/main/docs/01_refactor_lib_rpc_lib_web)
      - [Jeremy Chones video describing the PRs - 2 Cool Pull Requests for Rust Web App Blueprint](https://www.youtube.com/watch?v=MHwpSZA2uNA) 
    - [👉 PR: Adding gateway/worker architecture](https://github.com/vamsi-juvvi/rust-web-app/pull/1) needed the previously done refactoring split. I then proceeded to build a prototype _(which I use as my main testbed)_ that takes a `Gateway` which routes incoming RPC calls to different workers based on URL.
      - [Design docs](https://github.com/vamsi-juvvi/rust-web-app/tree/main/docs/02_worker_architecture)

 - **rust-genai**
   - [👉 PR: Adding function calling to OpenAI/Groq adapters ](https://github.com/vamsi-juvvi/rust-genai/pull/1) _this was ultimately not merged as the author Jeremy Chone wanted to go a different direction_
     - [👉 Documentation for get_weather tool calling example](https://github.com/vamsi-juvvi/rust-genai/blob/function_calling_openai/docs/add-function-calling/c06-code-and-traces.md) compares OpenAI's references python impl with the rust one I created.
     - [👉 Documentation for set_temperature IOT tool calling example](https://github.com/vamsi-juvvi/rust-genai/blob/function_calling_openai/docs/add-function-calling/c07-code-and-traces.md) a more complex example that requires the LLM to sequence two tool calls in the right order.
     - [Design/Code docs for the PR](https://github.com/vamsi-juvvi/rust-genai/blob/function_calling_openai/docs/add-function-calling/0-AddingFunctionCallingToGenAI.md) extensively documented this as it was an unsolicited PR into another author's repo and there were a lot of changes: I wanted to show that I had taken plenty of care to be a good guest.


### Rust - LLM - Pregel infrastructure to compose agentic nodes

 [LLM Agents.md](./LLM/LLM_Agents.md) includes some motivation behind `Pregel`. Essentially, a paralell graph processing algo out of Google from a while back. Caought references to it being the motivation behind `Langgraph` and `LlamaIndex` so decided to dig into it. Tons of great properties. Wanted to explore the dynamic graph morphing aspects of it so decided to build a version myself. Am listing the code in this repo for you to checkout.

  - [👉 lib-pregel](./Rust/agentic/lib-pregel/) is the basic `pregel` algorithm crate.
  - [👉 lib-agentic](./Rust/agentic/lib-agentic/) is the agentic layer crate on top of it. I was building up the pregel infra and capacilities as I add more complex examples.
    - [👉 example - one shot llm](./Rust/agentic/lib-agentic/examples/01_one_shot_llm.rs)
    - [👉 example - joke gen and critique](./Rust/agentic/lib-agentic/examples/02_joke_gen_and_critique.rs)
    - [👉 example - tool calling](./Rust/agentic/lib-agentic/examples/03_tool_calling.rs) this is built on top of the `genai` crate. I am however using a rust crate to automatically generate a json-schema from a struct such that my tools are plain rust lambdas that take an instance of the struct _(nicer ergonomics than dealing with raw json strings that come in for LLM tool calls)_   

Some notes about Pregel and it's advantages
  
   - Explored LangChain/LangGraph and LLamaIndex. Ultimately was disappointed, among other things _(shared input context with risk of races, outputs were dict slices into the same big dict, collision headaches)_, they had hidden the `Pregel` basis of their graphs. I thought this oversimplified the APIs and hid the possibilities of runtime graph evolution _(among other pregel properties)_.
   - Built my own `Pregel` implementation in rust sticking close to the original papers _(neo4j and databricks have their own APIs which were instructive)_
     - Pure message passing eliminates data races
     - Each `super-step` involves async execution of a subset of nodes and a join that waits for all. _(With a possibility of any-of, all-of, a/b in the join)_
     - All super-step outputs usable as graph-outputs instead of forcing a synthetic stop node into the graph.
     - Directly message any node (_not restricted to passing only along edges_)
     - Graph-level instrumentation to attach token/latency costs to edges and allow for graph algos to find shortest-paths/cheapest-cost at runtime based on server latency and such. Not sure if practical but the possibility exists if you don't hide the graph structure under your APIs.
     - Graph level observers, toppology mutators to perform dynamic insertion of nodes, optimization loops and things like that.

## Misc projects

- **Screenplay parsing** to parse loosely structured screenplays _(of actual movies)_ from the [HugginFace IMSDB dataset](https://huggingface.co/datasets/mattismegevand/IMSDb) into structured json so I can start using it for a **text -> animation** effort
   - [Convert IMSDB database/jsonl into individual files](https://colab.research.google.com/github/juvvination/juvvination.github.io/blob/master/nbs/Screenplay_Dataset_to_files.ipynb). The dataset is quite large and I do not have it on github. One sample file is listed at [./data/IMSDB/aladdin.txt](./data/IMSDB/aladdin.txt) and I will be using that in the next two notebooks.
   - [LLM - Parsing semi structured screenplay using OpenAI's structured outputs and Pydantic](https://colab.research.google.com/github/juvvination/juvvination.github.io/blob/master/nbs/LLM_StructuredOutput_Screenplay.ipynb)
   - [Antlr grammar - Parser to parse the screenplay files](https://colab.research.google.com/github/juvvination/juvvination.github.io/blob/master/nbs/StructuredScreenplay_Antlr.ipynb) OpenAI's APIs gave me `ContentFilter` errors. They mustve flagged it for copyright even though all I was asking for was parsing an existing screnplay. Instead of doing this formatting in chunks or utilizing a local LLM via Ollama etc, I decided to sharpen old Antlr parser generator skills and built a grammer to do this for me. _Just because a LLM can do the job does not mean it will actually do it. Sometimes traditional alternatives are needed_
     - [Parser grammar](./lib/python/imsdb/antlr/Screenplay.g4)
     - [README_DevelopingTheParser-2.md](./lib/python/imsdb/README_DevelopingTheParser-2.md)


 - More detailed acounts of my recent work summary   
   - [ExpandedRecentWorkSummary.md](./ExpandedRecentWorkSummary.md)
   - [Work summary limited to code](./CodeSamples.md)