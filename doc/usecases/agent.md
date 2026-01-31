# Agent Use Cases

<a id="UC-BUILD-KNOWLEDGE"></a>

## Build Knowledge Graph

The [ACT-AGENT (AI Agent)](../actors/system_users.md#ACT-AGENT) builds a GraphRAG knowledge base from the generated JSON graph.

Depends on: [ACT-AGENT (AI Agent)](../actors/system_users.md#ACT-AGENT), [UC-GRAPH (Generate Graph)](./analysis.md#UC-GRAPH)

<a id="UC-SUGGEST-USAGE"></a>

## Suggest Usage

The [ACT-AGENT (AI Agent)](../actors/system_users.md#ACT-AGENT) suggests how to use the system based on the constructed knowledge.

Depends on: [ACT-AGENT (AI Agent)](../actors/system_users.md#ACT-AGENT), [UC-BUILD-KNOWLEDGE (Build Knowledge Graph)](#UC-BUILD-KNOWLEDGE)
