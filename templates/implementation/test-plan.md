# Test Plan

> Use this template to define the testing strategy for a feature, endpoint, or component.

---

## Metadata

- **Feature/Spec Reference:** [Link or name of the specification being tested]
- **Status:** Draft | Ready | In Progress | Complete
- **Test Framework:** [e.g., Jest, pytest, cargo test — from Project Definition]

## Test Scope

### In Scope

- [e.g., All public API methods of UserService]
- [e.g., Request validation for POST /api/users]
- [e.g., Error handling for network failures]

### Out of Scope

- [e.g., Third-party OAuth provider behavior]
- [e.g., Database performance under load]

## Test Categories

### Unit Tests

| # | Test Name | Description | Input | Expected Result |
|---|-----------|-------------|-------|-----------------|
| 1 | [test_name] | [What it verifies] | [Input data] | [Expected output] |
| 2 | [test_name] | [What it verifies] | [Input data] | [Expected output] |

**Location:** [e.g., src/services/__tests__/user-service.test.ts]

### Integration Tests

| # | Test Name | Description | Setup | Expected Behavior |
|---|-----------|-------------|-------|-------------------|
| 1 | [test_name] | [What it verifies] | [Required setup] | [Expected behavior] |

**Location:** [e.g., tests/integration/user-api.test.ts]

### Edge Case Tests

| # | Test Name | Scenario | Input | Expected Behavior |
|---|-----------|----------|-------|-------------------|
| 1 | [test_name] | [Boundary condition] | [Input] | [How it handles it] |
| 2 | [test_name] | [Empty/null input] | [Input] | [How it handles it] |

### Error Case Tests

| # | Test Name | Scenario | Input | Expected Error |
|---|-----------|----------|-------|----------------|
| 1 | [test_name] | [Invalid input] | [Input] | [Error type/message] |
| 2 | [test_name] | [External failure] | [Setup] | [Error handling] |

## Test Data

### Fixtures / Mocks

| Name | Purpose | Location |
|------|---------|----------|
| [name] | [What it provides] | [file path or inline] |

### Test Constants

```
[Define any shared test data, factory functions, or mock responses]
```

## Environment Requirements

- [ ] [e.g., Test database running]
- [ ] [e.g., Mock server configured]
- [ ] [e.g., Environment variables set]

## Execution

### Run All Tests

```bash
[Command to run all tests for this feature]
```

### Run Specific Test

```bash
[Command to run a single test]
```

## Coverage Notes

[Any areas that are intentionally not covered and why, or areas that need manual testing.]
