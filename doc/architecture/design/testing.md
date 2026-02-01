<a id="CC_TESTING_STRATEGY"></a>

# Testing Strategy

We prioritize code quality and correctness through a multi-layered testing approach.

## Overview

| Type | Focus | Document |
| :--- | :--- | :--- |
| **Unit & Integration** | Correctness of individual functions and components. | [CC_UNIT_TESTING (Unit Testing Standards)](./unit-testing.md#CC_UNIT_TESTING) |
| **E2E** | Verification of user workflows (CLI, LSP) and system behavior. | [CC_E2E_TESTING (E2E Testing Design)](./e2e-testing.md#CC_E2E_TESTING) |
| **Performance** | Benchmarking and scalability verification. | [CC_PERF_TESTING (Performance Testing)](./performance-testing.md#CC_PERF_TESTING) |
| **Coverage** | Measurement of test effectiveness. | [CC_COVERAGE (Code Coverage Standards)](./coverage.md#CC_COVERAGE) |

## CI/CD Integration

All testing layers are integrated into our CI/CD pipeline to ensure that every pull request meets our quality standards.
