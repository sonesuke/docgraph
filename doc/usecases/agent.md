# Agent Use Cases

<a id="UC_BUILD_KNOWLEDGE"></a>

## Build Knowledge Graph

The [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT) builds a GraphRAG knowledge base from the generated JSON graph.

Depends on: [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT), [UC_GRAPH (Generate Graph)](./analysis.md#UC_GRAPH)

<a id="UC_SUGGEST_USAGE"></a>

## Suggest Usage

The [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT) suggests how to use the system based on the constructed knowledge.

Depends on: [ACT_AGENT (AI Agent)](../actors/systems.md#ACT_AGENT), [UC_BUILD_KNOWLEDGE (Build Knowledge Graph)](#UC_BUILD_KNOWLEDGE)
