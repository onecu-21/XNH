# XNH

**XNH's Not Human Architect**

> Autonomous product engineering, from intent to implementation.

XNH is an experimental open-source system for designing products with AI.

The goal is simple:

**Humans describe what they want. XNH figures out how to build it.**

Instead of requiring users to understand CAD, PCB routing, HDL, firmware, backend architecture, or manufacturing details, XNH aims to automate the engineering process and present the user with understandable results, trade-offs, and validation evidence.

## Vision

Traditional engineering workflows look like this:

```text
Idea
 ↓
Product planning
 ↓
Mechanical design
 ↓
Electronics
 ↓
PCB
 ↓
HDL
 ↓
Firmware
 ↓
Backend
 ↓
Testing
 ↓
Manufacturing
```

XNH aims for:

```text
Human Intent
     ↓
     XNH
     ↓
Validated Product
```

Internally, XNH may still perform all of the traditional engineering steps.

The difference is that **the human does not need to operate those tools directly.**

## Core Principles

### Humans specify intent

Users should describe goals and constraints rather than implementation details.

```text
Make it under 500 g.
Battery life must exceed 12 hours.
Keep the total cost below $500.
Performance must not decrease.
```

XNH translates those requirements into engineering constraints.

### AI designs, deterministic systems verify

AI may propose architectures, geometry, circuits, firmware, or software.

AI does **not** get to declare its own work correct.

```text
AI Proposal
    ↓
Design Engine
    ↓
Simulation / Verification
    ↓
Evidence
    ↓
Accept or Reject
```

Validation should come from deterministic tools wherever possible.

### The design is the source, not the generated files

STEP files, Gerbers, RTL, firmware, and backend code are treated as generated artifacts.

The canonical representation is a structured **Design Graph** containing:

* requirements
* parameters
* dependencies
* components
* constraints
* design decisions
* validation evidence
* manufacturing requirements
* revisions

### Changes propagate automatically

When part of a design changes, XNH should determine what is affected.

```text
Battery changed
    ↓
Mass changed
    ↓
Enclosure becomes stale
    ↓
Thermal model becomes stale
    ↓
Runtime estimate becomes stale
    ↓
Affected validations rerun
```

Previous validation evidence is preserved but marked stale when it no longer applies.

### No prompt engineering required

Users should not need to write elaborate AI prompts.

XNH should internally handle:

* model selection
* tool selection
* structured outputs
* context construction
* reasoning budgets
* retries
* validation
* model routing
* caching

The intended interface is ordinary language.

## Planned Architecture

```text
                    Human
                      │
                Product Intent
                      │
                      ▼
              Product Intent Compiler
                      │
                      ▼
                Architect AI
                      │
          ┌───────────┼───────────┐
          ▼           ▼           ▼
     Mechanical      PCB       Software
         AI           AI           AI
          │           │           │
          └───────────┼───────────┘
                      ▼
               Unified Design Graph
                      │
          ┌───────────┼───────────┐
          ▼           ▼           ▼
       CAD Core    PCB Core      EDA
          │           │           │
          ├───────────┼───────────┤
          │                       │
          ▼                       ▼
     Simulation              Build / Tests
          │                       │
          └───────────┬───────────┘
                      ▼
              Validation Evidence
                      │
                      ▼
                Design Revision
```

## Planned Domains

XNH is intended to eventually cover multiple engineering domains.

### Mechanical

* parametric geometry
* assemblies
* materials
* tolerances
* manufacturing constraints
* interference detection
* thermal and structural simulation integration
* process compensation
* STEP/STL/glTF export

### PCB

* board definition
* component placement
* routing
* stackups
* differential pairs
* impedance constraints
* DRC
* SI/PI validation
* Gerber generation
* BOM and pick-and-place generation

### Digital Hardware

* hardware architecture
* HDL generation
* simulation
* formal verification
* synthesis
* timing analysis
* place and route
* power and area optimization

Existing open-source EDA tooling may be integrated where appropriate.

### Firmware

* BSP and HAL generation
* peripheral configuration
* drivers
* RTOS tasks
* state machines
* communications
* power management
* bootloaders
* OTA
* automated testing
* hardware-in-the-loop validation

### Software and Backend

* API design
* database design
* authentication
* distributed systems
* device communication
* deployment
* observability
* testing
* infrastructure architecture
* cost optimization

## AI Backends

XNH is intended to support both remote and local AI models.

Possible modes include:

```text
XNH
 ├─ Remote AI APIs
 ├─ Local models
 └─ Self-hosted inference
```

The architecture should remain provider-independent while allowing optimized integrations for capable tool-using models.

## Technology

The current intended stack is:

```text
Core            Rust
Desktop UI      Electron + TypeScript
3D Viewer       Web technologies / GPU rendering
AI Interface    Tool calling / MCP / structured APIs
Design Storage  Versioned structured Design Graph
```

Rust is used for the core because XNH is expected to become a large, concurrent, cross-platform system where strong compile-time guarantees are useful.

Also:

> Using Rust is fucking awful, but having someone write it for you is beautiful.

## Project Status

**Very early development.**

XNH is currently an experimental project and should not be trusted for production engineering, safety-critical systems, manufacturing, or hardware design.

Expect major architectural changes.

## Non-Goals

XNH is not intended to become another traditional CAD application.

The project does not prioritize:

* complex manual CAD workflows
* professional human-oriented modeling UX
* thousands of toolbar buttons
* manual PCB routing as the primary workflow
* requiring users to understand generated implementation details

Humans should be able to inspect designs and ask questions about them, but XNH is primarily designed to be operated by AI.

## Example

Eventually, an interaction could look like:

```text
User:
Make this enclosure 20% lighter.
Do not reduce durability.
Do not increase manufacturing cost.

XNH:
Evaluated 184 design candidates.

Selected revision:
- Mass: -21.3%
- Estimated manufacturing cost: -0.7%
- Drop-test safety factor: 1.84
- Thermal performance: unchanged

All required validations passed.
```

No CAD expertise required.

## License

XNH is licensed under the **GNU General Public License v3.0 or later**.

See `LICENSE` for details.

## Name

**XNH** is a recursive acronym:

> **XNH's Not Human Architect**

Because the architect isn't human.
