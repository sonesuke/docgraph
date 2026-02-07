# Structure Definition Use Cases

<a id="UC_WRITE"></a>

## Write Specifications

The developer writes specifications in Markdown using anchor heading format.

### Actors

- [ACT_DEV (Developer)](../actors/users.md#ACT_DEV)

### Interfaces

- [IF_CONFIG (docgraph.toml Configuration)](../requirements/interfaces/interfaces.md#IF_CONFIG)

### Requirements

- [FR_CORE_UNIQUE (Unique Node IDs)](../requirements/functional/core.md#FR_CORE_UNIQUE) Preventing ambiguity in requirement references
- [FR_CORE_VALID_REF (Valid References)](../requirements/functional/core.md#FR_CORE_VALID_REF) Ensuring internal consistency of the documentation graph

### Flow

1. Developer opens a Markdown file.
2. Developer adds an anchor tag before a heading.
3. Developer defines properties or relationships using Markdown links.

### Non-Functional Requirements (Optional)

- [NFR_VSCODE_PORTABILITY (Cross-platform Portability)](../requirements/non-functional/vscode.md#NFR_VSCODE_PORTABILITY) Ensures consistent specification writing experience on all platforms
