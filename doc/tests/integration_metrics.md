
# Test Specifications

```{document} Lint Integration Test
:id: TEST-INT-LINT
:kind: test
:verifies: SPEC-CLI-LINT

Run `docgraph lint` (defined in {ref}`SPEC-CLI-LINT`) on a known defective graph and verify it reports errors.
```

```{document} Gen Integration Test
:id: TEST-INT-GEN
:kind: test
:verifies: SPEC-CLI-GEN

Run `docgraph gen` (defined in {ref}`SPEC-CLI-GEN`) on a sample graph and verify the JSON output matches the expected structure.
```
