> Angadi asked me to write up a one-page doc about what I have been upto. Will start and then reduce it as needed.

<!-- TOC -->

- [OSS contributions - Rust and Docs](#oss-contributions---rust-and-docs)
- [LLM - TANL - Python notebook](#llm---tanl---python-notebook)
- [LLM - Pregel infrastructure to compose agentic nodes - Rust](#llm---pregel-infrastructure-to-compose-agentic-nodes---rust)
- [Frontend](#frontend)

<!-- /TOC -->
---

## OSS contributions - Rust and Docs

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


## LLM - TANL - Python notebook

During my NLP studies, I came across this interesting paper out of Amazon [Structured prediction as translation between augmented natural Languages (TANL)](https://arxiv.org/pdf/2101.05779) that modeled traditional NLP problems as a seq-to-seq translation [amazon-science/tanl](https://github.com/amazon-science/tanl) that tried to implement traditional NLP tasks _(NER, SRL, POS, CoRef)_ via a small LM. The goal was to achieve low latency and making use of existing pre-trained knowledge of the LM, fine tune it for NLP tasks (single or multi-task).

The github code was related to evaluating against benchmarks, but my goal was to see if I could extract it out as a library that could run as an NLP service _(behind a JsonRPC API)_. I also wanted to extract out their fine-tuning training data and see if I can achieve any better results with a more modern 2B class LM like Qwen or LLama. The following links list my notes and notebooks that help me in this.

 - [👉 Github Fork - Notes, Plan and Progress on use of TANL research code](https://github.com/vamsi-juvvi/tanl/blob/main/notebooks/TANL.md)
 - [👉 Github Fork - Collection of notebooks to explore TANL annotations](https://github.com/vamsi-juvvi/tanl/tree/main/notebooks)

>Basic NLP tasks can still be relevant in the LLM era. You want low latency, predictability vs creativity and synthesis: however, you can take advantage of language understanding baked into the SLM/LLM. Since you typically have task-specific fine-tuning _(POS, NER, CoRef, SRL etc)_, it is worth a look to see if a multi-headed approach can put each of these tasks into separate fork all sharing the same backbone. The hope is that a single forward pass will yield multiple task-specific outcomes and avoid the latency/cost of multiple forward passes.
>
> NLP tasks like NER are also valuable in RAG for better query embedding.

## LLM - Pregel infrastructure to compose agentic nodes - Rust

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
     - Direct messaging outside of edge-only
     - graph-level instrumentation to attach token/latency costs to edges and allow for graph algos to find shortest-paths/cheapest-cost at runtime based on server latency and such. Not sure if practical but the possibility exists if you don't bury the graph structure under your APIs.
      
## Frontend

After investing an inordinate amount of time in ScalaJS, Laminar and ScalablyTyped _(for converting typescript into ScalaJS)_, I decided to switch to a best-of-breed tool for UI and settled on React. React/TS for the web and ReactNative on mobile. This was a whole new ecosystem with Js, Ts, runtime bundlers, CORS and such but I managed to build myself a React front end with basic authentication setup to talk to a rust-web based server via JsonRPC calls. 

[👉 Code - The React Frontend which talks to my rust web-server](./React/frontend/)