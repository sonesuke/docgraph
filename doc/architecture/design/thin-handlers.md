<a id="CC_THIN_HANDLERS"></a>

# Thin Handlers

## Overview

The Thin Handlers principle states that **handlers should contain no business logic**. They serve only as adapters between the user interface and the Core layer.

## The Three Responsibilities

Handlers perform exactly three tasks:

1. **Input transformation**: Convert UI-specific types to Core types
2. **Core invocation**: Call Core logic functions
3. **Output transformation**: Convert Core types back to UI-specific types

## Related

- [ADR_THIN_HANDLERS (Thin Handlers)](../../decisions/thin-handlers.md#ADR_THIN_HANDLERS)
