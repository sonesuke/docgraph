---
name: compose
description: Schema Composition - Design the graph topology and generate docgraph.toml for deterministic traversal.
---

# Schema Composition

This skill creates the normative topology of the documentation graph by generating `docgraph.toml`.

The schema defines what can exist, how it can connect, and what causal traversal means.

> [!IMPORTANT] The schema must minimize ambiguity while allowing evolution. Restrict relations by direction and target
> types to enforce determinism.

---

## Environment Bootstrap

`compose` assumes the documentation graph can be observed and validated. If `docgraph` is not available, the agent MUST
install it.

The graph system cannot exist without the graph tool.

### Install `docgraph`

macOS / Linux:

```bash
curl -fsSL https://raw.githubusercontent.com/sonesuke/docgraph/main/install.sh | bash
```

Windows (PowerShell):

```powershell
powershell -c "irm https://raw.githubusercontent.com/sonesuke/docgraph/main/install.ps1 | iex"
```

### Verify installation

```bash
docgraph --version
```

If the command fails, the agent MUST stop and report environment failure.

---

## Core Axiom

The schema must minimize ambiguity while preserving evolution.

Too strict → the graph cannot grow. Too loose → `align` cannot determine unique meaning.

Every schema decision is a trade-off between these two forces.

The schema is the only place where new semantics are introduced. All other skills only enforce or traverse what the
schema defines.

---

## Outputs

- `docgraph.toml` (the sole normative source)
- Minimal templates per node type (optional, in `doc/templates/`)

---

## 0. Inputs

Before composing, the agent MUST establish:

- **Domain**: What kind of system or product is being documented?
- **Terminal nodes**: What counts as "done"? (Implementation? Tests? Metrics?)
- **Primary reasoning direction**: Top-down (Why→How) or bottom-up (How→Why)?

If these inputs are unavailable, the agent MUST assume defaults and state them explicitly.

---

## 1. Role Inventory

Define the reasoning roles required by the project.

Minimum viable set:

| Role           | Causal Question | Required |
| :------------- | :-------------- | :------- |
| Intent         | Why?            | Yes      |
| Responsibility | What?           | Yes      |
| Realization    | How?            | Yes      |
| Evidence       | Proven?         | No       |
| Constraint     | Boundary?       | No       |
| Rationale      | Justified?      | No       |
| Domain         | Context?        | No       |

Intent, Responsibility, and Realization are always required. The others depend on project maturity.

---

## 2. Type Mapping

Map concrete node types to roles. The mapping MUST be explicit and complete.

> [!NOTE] Node types are schema-specific. The skill must work for any naming scheme.

Every node type MUST belong to exactly one role. If a type spans two roles, the schema is ambiguous.

### Example: Product Spec World (non-normative)

| Role           | Node Types   |
| :------------- | :----------- |
| Intent         | UC, ACT      |
| Responsibility | FR, NFR      |
| Realization    | MOD, IF, CC  |
| Evidence       | TEST, METRIC |
| Constraint     | CON          |
| Rationale      | ADR          |
| Domain         | DAT          |

---

## 3. Relation Primitives

Define a small controlled set of relation types (`rel`) that correspond to causal questions. `rel` becomes `r.type` in
Cypher.

> [!IMPORTANT] `rel` is a controlled vocabulary. Any `rel` not declared here is invalid.

Recommended minimal set:

| Relation       | From Role      | To Role        | Question                |
| :------------- | :------------- | :------------- | :---------------------- |
| refines        | Intent         | Responsibility | What must be satisfied? |
| realized_by    | Responsibility | Realization    | How is it achieved?     |
| constrained_by | Responsibility | Constraint     | What boundaries apply?  |
| justified_by   | Realization    | Rationale      | Why this design?        |
| verified_by    | Realization    | Evidence       | Is it proven?           |
| depends_on     | Realization    | Realization    | What depends on this?   |
| defines        | Domain         | any            | What context applies?   |

Each relation MUST have a clear causal direction. Bidirectional relations indicate ambiguity.

---

## 4. Target Restrictions

For each relation type, restrict allowed source and target node types.

This is where determinism is enforced. The narrower the targets, the less `align` has to guess.

Example constraints in `docgraph.toml`:

```toml
[nodes.FR]
desc = "Functional Requirement"
template = "doc/templates/functional.md"
rules = [
  { dir = "to", targets = ["MOD"], min = 1, desc = "Must be realized", rel = "realized_by" },
]
```

### Rule Syntax

Each rule has the following fields:

| Field     | Description                                                                    |
| :-------- | :----------------------------------------------------------------------------- |
| `dir`     | `"to"` (this node references target) or `"from"` (target references this node) |
| `targets` | Array of allowed node type IDs, or `["*"]` for any type                        |
| `min`     | Minimum required count (`0` = optional, `1+` = required)                       |
| `desc`    | Human-readable justification for the rule                                      |
| `rel`     | Relation type identifier, snake_case. Used as `r.type` in Cypher queries       |

### Special Patterns

**`dir = "from"`**: Declares an _inbound_ expectation. The rule says "this node expects to be referenced by targets".
Use when the _target_ side owns the link in the markdown.

**`targets = ["*"]`**: Accepts references from any node type.

> [!CAUTION] `"*"` disables type-level restriction. If overused, `align` cannot determine causal role from relations
> alone.

> [!IMPORTANT] `targets=["*"]` is allowed, but it shifts the burden to determinism. When using `targets=["*"]`, the
> schema MUST ensure role determinism using one of:
>
> - the source node type has a unique role by definition, OR
> - the `rel` itself implies a unique causal question independent of target type, OR
> - an additional rule narrows interpretation (e.g., required inbound/outbound constraints).

### Determinism Rule (for `targets=["*"]`)

If `targets=["*"]` is used, the schema MUST state why interpretation remains unambiguous. This justification MUST
reference Role Inventory and Relation Primitives.

Design principles:

- Every Responsibility MUST reach at least one Realization
- Realization SHOULD be justifiable (link to Rationale)
- Constraints MUST target Responsibility, not Realization directly

---

## 5. Closure Requirements

Verify that the schema allows valid traversal from Intent to terminal nodes.

The agent MUST check:

- Every Responsibility type has at least one path to a Realization type
- If Evidence types exist, at least one Realization type can reach them
- No role is completely isolated (unreachable from any other role)

If closure fails, the schema has a structural gap.

---

## 6. Template Generation

Each node type in `docgraph.toml` SHOULD have a `template` field pointing to a template file.

Templates define the minimal document structure for a node type. They ensure new nodes are created with the correct
anchor, required sections, and placeholder links.

### Template Syntax Rules

**Anchor** (required, first line):

```markdown
<a id="{TYPE}_*"></a>
```

`*` is replaced with the actual ID suffix when instantiating (e.g., `FR_001`).

**Heading level**: Match the document nesting depth.

- Top-level documents: `# {Title}` or `## {Title}`
- Nodes embedded in a shared file: `### {Title}`

**Link placeholder format**:

```markdown
[{TARGET*TYPE}* (\_)](*#{TARGET_TYPE}_*)
```

- `{TARGET_TYPE}*` → type prefix + wildcard ID (e.g., `MOD*`)
- `(*)` → placeholder display name
- `(*#{TARGET_TYPE}_*)` → placeholder anchor reference

**Required sections** (`min >= 1` rules): Always include. Name the section after the `rel` value.

**Optional sections** (`min = 0` rules): Include with `(Optional)` suffix in the heading.

> [!NOTE] Template examples below are illustrative. Replace section headings with the `rel` values declared in your
> schema.

### Template Patterns

**Rich node** (has outbound rules): Include link sections per rule.

```markdown
<a id="{TYPE}_*"></a>

## {Title}

{Description}

### <rel>

- [<TARGET*TYPE>* (\_)](*#<TARGET_TYPE>_*)

### <rel> (Optional)

- [<TARGET*TYPE>* (\_)](*#<TARGET_TYPE>_*)
```

**Leaf node** (only inbound `from` rules, no outbound): Minimal template with no link sections.

```markdown
<a id="MOD_*"></a>

### {Title}

{Description}
```

**Rationale node** (ADR): Domain-specific sections instead of link sections.

```markdown
<a id="ADR_*"></a>

# {Title}

{Description}

## Decision

{Decision}

## Rationale

{Rationale}
```

### Registering Templates

The `template` field in `docgraph.toml`:

```toml
[nodes.UC]
desc = "Use Case"
template = "doc/templates/usecases.md"
```

---

## 7. Generate `docgraph.toml`

Emit the schema. This file becomes the sole normative source for all other skills.

### `[graph]` Section

Define global graph settings before node definitions. The `ignore` field excludes paths from graph traversal.

```toml
[graph]
ignore = ["README.md", "SECURITY.md", "doc/templates"]
```

Typical ignore targets:

- Non-document files (README, SECURITY, CHANGELOG)
- Template directories (templates are blueprints, not graph nodes)
- Plugin/tool configuration directories

### `[nodes.*]` Sections

Emit one section per node type, following the Type Mapping and Target Restrictions defined above.

Emit the schema. This file becomes the sole normative source for all other skills.

After generation:

1. Run `docgraph check` to verify the schema and graph loader are consistent.
2. If existing documents/nodes exist:
   - Run `validate` on a representative node of each major role.
   - Run `align` on at least one node per role boundary (e.g., Intent/Responsibility, Responsibility/Realization).
3. If no nodes exist yet:
   - Generate one minimal stub per required role using templates, then run `validate` and `align`.

---

## What Compose Does NOT Do

Compose does not:

- Write document content
- Validate existing nodes
- Judge alignment or reasoning correctness

Compose creates the rules. Other skills enforce them.
