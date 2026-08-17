# Smithers

> A declarative session rehydration system for Linux.

It restores your complete development context from a project directory—processes, terminals, editors, browsers, and supporting tools—so you can continue working immediately after a reboot, switch, or interruption.

Smithers does not replace your tools.

It orchestrates them.

---

## The Problem

Every software project quietly accumulates a working environment:

* a terminal running build/watch commands
* a Neovim or Emacs session
* a database or cache process
* a browser on local documentation
* a log viewer
* a debugger session
* background services

Rebuilding this context manually is slow, repetitive, and fragile.

Most tools solve parts of this problem:

* tmux restores terminal layouts
* Docker restores runtime environments
* IDEs restore editor state
* window managers restore layouts

None of them restore the full working context.

---

## The Solution

Smithers defines a single concept:

A session is a declarative graph of processes and their relationships.

From this graph, Smithers:

* starts processes
* manages dependencies
* restores terminals and editors
* launches browsers and tools
* coordinates startup order
* attaches to existing sessions when possible

Then it stops.

---

## Philosophy

Smithers follows the Unix philosophy:

* It composes existing tools
* It avoids reimplementing functionality
* It stays transparent and inspectable
* It remains deterministic and declarative
* It never becomes an IDE or a runtime environment

Smithers orchestrates.

It does not own.

---

## Non-Goals

Smithers will never:

* compile code
* lint code
* test code
* manage dependencies
* scaffold projects
* replace Docker or Dev Containers
* replace tmux, WezTerm, or terminal emulators
* replace Neovim or any editor
* replace window managers (i3, AwesomeWM, etc.)
* become an IDE
* become a task runner or CI system

If a feature drifts into these areas, it belongs in an existing tool.

---

## Design Principles

Every feature must satisfy:

1. Does this reduce the cognitive cost of re-entering a project?
2. Can this be expressed declaratively?
3. Is Smithers still only orchestrating?
4. Can an existing tool already do this better?
5. Will this still make sense if I switch editor, terminal, or window manager?

If not, it does not belong in Smithers.

---

## How It Works

A project defines a single session file:

.smithers.toml

Smithers parses it into a session graph:

* nodes: processes, terminals, applications
* edges: dependencies and startup order
* metadata: working directories, environment variables, roles

Example (conceptual):

```toml
[[node]]
name = "editor"
type = "process"
command = "nvim"
[[node]]
name = "server"
type = "process"
command = "cargo run"
[[node]]
name = "browser"
type = "browser"
url = "http://localhost:3000"
depends_on = ["server"]
```

---

## Execution Model

Smithers does not reimplement applications.

It:

* spawns subprocesses
* wraps existing CLI tools
* integrates with system interfaces where needed
* delegates persistence to the underlying tools

Examples of integrations:

* Neovim
* WezTerm
* tmux
* i3 / AwesomeWM
* LibreWolf / Firefox
* Docker / Dev Containers
* make / just / cargo / npm

---

## Core Command

`smithers continue`

This command:

1. Reads .smithers.toml
2. Builds a session graph
3. Determines execution order
4. Restores or launches all nodes
5. Attaches to running processes where possible

Result:

Your working environment is restored.

---

Key Property: Determinism

Running:

smithers continue

should always produce the same session graph behavior.

If something changes, it should be visible in the configuration—not hidden logic.

---

## Failure Model

Smithers prefers partial success over total failure.

If one node fails:

* others still start
* errors are reported clearly
* system remains usable

---

Why Rust

Smithers benefits from Rust because it requires:

* deterministic execution
* safe process orchestration
* structured graph modeling
* predictable CLI behavior
* strong type guarantees for session definitions

---

## Status

Early design phase.

The goal is not feature completeness.

The goal is a clean, minimal orchestration core that can reliably restore a working developer environment.

---

## Installation
```bash
curl -sSL https://raw.githubusercontent.com/Sharko64/smithers/main/scripts/install.sh | less
```

---

## Funding
![Bitcoin](https://img.shields.io/badge/Bitcoin-000?style=for-the-badge&logo=bitcoin&logoColor=orange) <br>
![Ethereum](https://img.shields.io/badge/Ethereum-3C3C3D?style=for-the-badge&logo=Ethereum&logoColor=white) <br>
![Solana](https://img.shields.io/badge/solana-%239945FF.svg?style=for-the-badge&logo=solana&logoColor=white)
