# The Dependency Rule

<a id="CC_DEPENDENCY_RULE"></a>

## The Dependency Rule

Source code dependencies must point only inward, toward higher-level policies.

**Rules:**

1.  Nothing in an inner circle (Core) can know anything at all about something in an outer circle (Handlers, UI).
2.  The name of something declared in an outer circle must not be mentioned by the code in the an inner circle.

### Decided by

- [ADR_LAYERED_ARCH (Layered Architecture)](../../decisions/layered-architecture.md#ADR_LAYERED_ARCH)

### Reflected in (Optional)

- [NFR_EXT (Modular Design)](../../requirements/non-functional/extensibility.md#NFR_EXT)
