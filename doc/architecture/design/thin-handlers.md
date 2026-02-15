# Thin Handlers

<a id="CC_THIN_HANDLERS"></a>

## Thin Handlers Pattern

Request handlers should be "thin," delegating business logic to core modules.

**Responsibilities:**

1.  **Extract**: Deserialize and validate request parameters.
2.  **Delegate**: Call the appropriate core service/function.
3.  **Response**: Map the core result to a JSON RPC response.

**Implementation Details:**

1. **Input transformation**: Convert UI-specific types to Core types
2. **Core invocation**: Call Core logic functions
3. **Output transformation**: Convert Core types back to UI-specific types

### Decided by

- [ADR_THIN_HANDLERS (Thin Handlers)](../../decisions/thin-handlers.md#ADR_THIN_HANDLERS) To maintain separation of
  concerns.
