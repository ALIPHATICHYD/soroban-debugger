# GitHub Issue Drafts

These drafts are ready to paste into GitHub issues for `Timi16/soroban-debugger`.

## 1. Implement true source-level line breakpoints and stepping

**Title**

Implement true source-level line breakpoints and source stepping

**Body**

### Summary

The VS Code extension now supports a pragmatic breakpoint mode by mapping a clicked Rust source line to the surrounding exported contract function and stopping at that function's entry. This is better than the previous fake "verified" breakpoint behavior, but it is still not true line-level debugging.

### Current behavior

- VS Code source breakpoints are resolved to exported contract function entrypoints.
- The backend can pause before executing that function and resume on `continue`.
- Arbitrary line breakpoints inside a function body are not honored at the exact line.
- Step/next/step-in/step-out are not true source-level stepping operations.

### Why this matters

The extension can now honestly support function-entry breakpoints, but developers still expect:

- pause on the exact Rust source line they clicked
- line-by-line stepping
- accurate source stack frames and locations

### Suggested direction

- Connect DWARF/source-map data from `src/debugger/source_map.rs` into execution control.
- Track executable instruction offsets during contract execution.
- Support mapping from source lines to executable offsets.
- Add true pause/resume behavior at source locations, not only function entry.

### Acceptance criteria

- A breakpoint set on an executable Rust line pauses on that exact line.
- `continue` resumes until the next exact line breakpoint.
- `next`, `step in`, and `step out` behave as debugger users expect.
- Stack frames expose real file/line locations.

## 2. Implement `repl` subcommand

**Title**

Implement the `repl` subcommand

**Body**

### Summary

The `repl` command is wired into the CLI but currently returns `REPL mode is not yet implemented in this build`.

### Reproduction

```bash
./target/debug/soroban-debug repl --contract tests/fixtures/wasm/counter.wasm
```

### Current output

```text
REPL mode is not yet implemented in this build
```

### Expected behavior

An interactive REPL should allow:

- loading a contract
- calling exported functions
- inspecting storage/state
- reusing prior arguments and results
- exiting cleanly

### Acceptance criteria

- `repl` launches successfully for a valid WASM contract.
- Users can invoke exported functions interactively.
- Storage/state inspection commands work.
- The command has automated coverage beyond parser-only tests.

## 3. Implement `symbolic` subcommand

**Title**

Implement the `symbolic` subcommand

**Body**

### Summary

The `symbolic` command exists in the CLI surface but currently exits with `Symbolic mode not yet implemented`.

### Reproduction

```bash
./target/debug/soroban-debug symbolic --contract tests/fixtures/wasm/counter.wasm --function increment
```

### Current output

```text
Symbolic mode is not yet implemented in this build
```

### Expected behavior

The command should explore contract input space for a target function and emit useful scenarios, constraints, or counterexamples.

### Acceptance criteria

- `symbolic` can run on a valid contract/function pair.
- It produces deterministic output or a saved scenario/report.
- Failure modes are documented for unsupported types or missing metadata.

## 4. Implement `analyze` subcommand

**Title**

Implement the `analyze` subcommand

**Body**

### Summary

The `analyze` command is present but currently exits with `Analyze mode not yet implemented`.

### Reproduction

```bash
./target/debug/soroban-debug analyze --contract tests/fixtures/wasm/counter.wasm
```

### Current output

```text
Analyze mode is not yet implemented in this build
```

### Expected behavior

The command should run static and/or dynamic analysis against a contract and return actionable findings.

### Acceptance criteria

- `analyze` runs on a valid contract file.
- It emits findings in human-readable form.
- Output format and severity are stable enough for documentation/tests.

## 5. Implement `scenario` subcommand

**Title**

Implement the `scenario` subcommand

**Body**

### Summary

The `scenario` command is exposed in the CLI but currently exits with `Scenario mode not yet implemented`.

### Reproduction

```bash
./target/debug/soroban-debug scenario \
  --scenario examples/contracts/simple-token/scenario.toml \
  --contract tests/fixtures/wasm/counter.wasm
```

### Current output

```text
Scenario mode is not yet implemented in this build
```

### Expected behavior

The command should execute a TOML scenario file against a contract and report each step's result.

### Acceptance criteria

- Valid scenario files execute end to end.
- Failures identify the specific step and reason.
- Example scenarios in the repo can be used as regression tests.

## Command status snapshot

- `remote`: working end to end against a live local server
- `completions`: working
- `repl`: not implemented
- `symbolic`: not implemented
- `analyze`: not implemented
- `scenario`: not implemented
