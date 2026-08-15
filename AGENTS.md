# XNH Agent Instructions

## Project

XNH stands for:

> **XNH's Not Human Architect**

XNH is an experimental autonomous product engineering system.

The long-term goal is to allow a human to describe product intent while AI systems perform the engineering work required to produce a validated implementation.

Eventually, XNH may cover:

* mechanical CAD
* PCB design
* HDL and EDA
* firmware
* backend and software architecture
* manufacturing
* simulation and validation

However, **do not attempt to build all of this at once.**

The project must grow incrementally.

---

# Core Development Philosophy

## 1. Build the smallest useful vertical slice

Never implement large future subsystems merely because they appear in the roadmap.

Implement only what is required for the current task.

Prefer:

```text
small working system
```

over:

```text
large incomplete architecture
```

Avoid speculative infrastructure.

---

## 2. Do not overengineer

Do not introduce abstractions for hypothetical future requirements.

Avoid unnecessary:

* traits
* factories
* plugin systems
* dependency injection
* generic frameworks
* dynamic dispatch
* macro systems
* configuration layers
* service abstractions

unless the current implementation genuinely requires them.

A direct implementation is preferred when it is sufficient.

---

## 3. Strong types over strings

XNH's core should use strongly typed Rust structures.

Prefer:

```rust
struct NodeId(Uuid);
struct RevisionId(Uuid);
struct ParameterId(Uuid);
```

over passing raw strings everywhere.

Represent domain concepts explicitly.

Invalid states should be difficult to represent.

---

## 4. AI proposes; deterministic systems verify

AI-generated output must not automatically be considered correct.

The long-term architecture should follow:

```text
AI proposal
    ↓
deterministic implementation
    ↓
validation
    ↓
evidence
    ↓
accept / reject
```

Do not design systems where an LLM can declare its own work valid without external verification.

---

## 5. Design data is the source of truth

Generated artifacts should eventually be considered outputs.

Examples:

* STEP
* STL
* Gerber
* HDL
* firmware
* backend source code

The canonical design should instead live in structured XNH representations such as the Design Graph and domain IRs.

Do not make generated artifacts the authoritative internal state unless the current task explicitly requires it.

---

# Current Development Priority

The first domain being developed is:

> **Mechanical CAD**

The immediate architecture is intentionally small:

```text
Design Graph
    ↓
Geometry IR
    ↓
Geometry implementation
    ↓
3D representation
```

Do not add PCB, EDA, firmware, backend generation, FEA, CFD, or manufacturing systems until explicitly requested.

---

# Current Rust Architecture

The intended initial workspace contains:

```text
xnh-core
xnh-graph
xnh-geometry
xnh-cli
```

## xnh-core

Shared fundamental types.

Keep this crate small.

Do not turn it into a dumping ground.

---

## xnh-graph

Contains the XNH Design Graph.

Initial responsibilities may include:

* typed node IDs
* nodes
* parameters
* dependencies
* structured serialization

Future concepts such as:

* revisions
* invalidation
* decisions
* evidence
* provenance

must only be implemented when explicitly requested.

---

## xnh-geometry

Contains geometry-related intermediate representations and logic.

Initial geometry should remain deliberately limited.

Start with simple operations such as:

* Box
* Cylinder
* Translate
* Rotate

Do not implement a full CAD kernel prematurely.

---

## xnh-cli

Provides a minimal executable interface for development and testing.

It may be used to:

* create example designs
* serialize structures
* invoke experimental functionality
* inspect results

It is not currently intended to become a complete user-facing CLI.

---

# Rust Rules

Use stable Rust unless there is a strong reason otherwise.

Prefer:

* ownership
* explicit types
* enums
* `Result`
* small modules
* clear data flow
* compile-time guarantees

Avoid unnecessary `unsafe`.

If `unsafe` is required:

1. keep the unsafe block as small as possible
2. document why it is safe
3. add tests around the boundary

---

# Dependencies

Keep dependencies minimal.

Before adding a dependency, ask:

1. Is it actually needed now?
2. Is implementing the required functionality ourselves unreasonable?
3. Is the crate maintained?
4. Does its license fit XNH?
5. Does it significantly simplify the implementation?

Do not add large frameworks for trivial tasks.

Common small dependencies such as these are acceptable when useful:

* `serde`
* `serde_json`
* `uuid`
* `thiserror`

Do not add dependencies merely for convenience without justification.

---

# Licensing

XNH is intended to use:

> **GPL-3.0-or-later**

Do not introduce code or dependencies whose licensing would conflict with distribution under GPL-3.0-or-later.

When uncertain about dependency licensing, flag it instead of guessing.

---

# Code Quality

Code should be understandable before it is clever.

Prefer:

```rust
fn validate_node(...)
```

over dense generic abstractions that save only a few lines.

Names should describe domain meaning.

Comments should explain **why**, not repeat **what the code says**.

---

# Testing

Every meaningful core behavior should have tests.

Before considering a task complete, run:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Fix failures introduced by the task.

Do not silence warnings merely to make CI green unless the warning is genuinely inappropriate.

---

# Scope Control

This rule is critical:

> **Do not implement future roadmap features unless explicitly requested.**

In particular, do not spontaneously add:

* OpenAI API integration
* other LLM providers
* MCP
* local model runtimes
* agent frameworks
* Electron
* GUI systems
* HTTP servers
* databases
* cloud services
* PCB design
* EDA
* HDL generation
* firmware generation
* backend generation
* FEA
* CFD
* manufacturing simulation
* distributed computation
* plugin ecosystems

even though they are planned long-term.

---

# Repository Changes

Before making significant changes:

1. inspect the repository
2. understand the existing architecture
3. identify the minimum affected files
4. preserve existing working behavior unless the task requires otherwise

Avoid large unrelated refactors.

If a task exposes an architectural problem, mention it instead of silently rewriting unrelated parts of the project.

---

# Working Style

For each task:

1. inspect relevant files
2. briefly determine the smallest viable implementation
3. implement it
4. format the code
5. run Clippy
6. run tests
7. inspect the resulting diff
8. summarize what changed

Do not claim tests passed unless they were actually executed successfully.

---

# Architecture Discipline

Keep module boundaries explicit.

Avoid circular dependencies between crates.

Lower-level crates should not depend on UI, network, or AI-specific layers.

A preferred dependency direction is:

```text
             xnh-cli
             /     \
            v       v
      xnh-graph   xnh-geometry
            \       /
             v     v
             xnh-core
```

Adjust this only when domain responsibilities justify it.

Do not create cyclic crate dependencies.

---

# Performance

Correctness and architecture come before premature optimization.

However:

* avoid obviously unnecessary copies
* avoid pathological algorithms
* avoid serializing/deserializing repeatedly without reason

Do not add complex optimization machinery until measurements justify it.

---

# AI-Specific Design Principle

XNH is eventually intended to be operated primarily by AI systems rather than human CAD experts.

Therefore APIs should favor:

* deterministic behavior
* explicit schemas
* structured errors
* machine-readable outputs
* stable identifiers
* clear preconditions
* clear postconditions

Prefer:

```text
InvalidWallThickness {
    minimum: 1.2,
    actual: 0.8
}
```

over:

```text
"Something went wrong."
```

Human-facing UX is not the current priority.

---

# When Unsure

If multiple implementations are possible, prefer the one that is:

1. smaller
2. easier to understand
3. easier to test
4. less coupled
5. easier to replace later

Do not solve problems XNH does not have yet.

---

# Final Principle

XNH is ambitious enough already.

Do not make it more complicated than necessary.

> **Build one correct layer at a time.**
