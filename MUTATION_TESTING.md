# Mutation Testing Guide

This document describes how to run mutation testing on the OurDAO contracts using cargo-mutants.

## What is Mutation Testing?

Mutation testing is a technique that evaluates test suite quality by introducing small, deliberate code mutations and verifying that the test suite catches them. Surviving mutants indicate potential gaps in test coverage.

## Running Mutation Tests

### Prerequisites

Install cargo-mutants:

```bash
cargo install cargo-mutants
```

### Basic Usage

Run mutation tests on the DAO contract:

```bash
cd contracts/dao
cargo mutants
```

### Generate Detailed Report

Generate an HTML report of surviving mutants:

```bash
cargo mutants --output html
```

The report will be generated in `mutants.html`.

### Run with Custom Options

```bash
# Run mutations on a specific crate
cargo mutants --package ourdao-dao

# Run with verbose output
cargo mutants -v

# Run with a custom timeout
cargo mutants --timeout 120

# Run specific mutation operators
cargo mutants --only "ArithmeticOperator"
```

## Configuration

The `mutants.toml` file in the repository root controls cargo-mutants behavior:

- **exclude**: Patterns to exclude from mutation
- **timeout**: Test execution timeout in seconds
- **skip_operators**: Mutation operators to skip

## Interpreting Results

### Undetected Mutants

Mutants that survive (are not caught by tests) indicate one of:

1. **Dead code paths** - Code that's never executed in tests
2. **Insufficient assertions** - Logic checked but assertions are too weak
3. **Missing error case tests** - Error conditions not tested

### Addressed Mutants

All detected mutants should be addressed by either:

1. **Adding tests** - Write tests to catch the mutation
2. **Refactoring code** - Simplify or remove unnecessary code
3. **Documenting** - Mark as known acceptable gap (rare)

## Error Coverage Tests

The `error_coverage.rs` test module provides comprehensive testing for all 29 error variants in the contract. These tests ensure that:

- Every error path is reachable
- Errors are properly triggered by contract logic
- Error messages are meaningful

This high error coverage helps mutation testing detect edge cases and error-handling logic properly.

## CI Integration

Mutation testing can be integrated into CI by adding a workflow step:

```yaml
- name: Run mutation tests
  run: cargo mutants --timeout 120
```

## Resources

- [cargo-mutants Documentation](https://github.com/sourcefrog/cargo-mutants)
- [Mutation Testing Best Practices](https://stryker-mutator.io/)
