% SMITHERS(1) smithers 1.0.0
% smithers manual
% Minimal. Opinionated. Industrial.

# NAME

**smithers** — project boilerplate generator for Bash, Python, and Rust

# SYNOPSIS

smithers new  [type]  [OPTIONS]

smithers list

smithers doctor

smithers –help
smithers –version

# DESCRIPTION

**smithers** is a minimal, standards-compliant project scaffolding tool for generating boilerplate templates for Bash, Python, and Rust projects.

It supports draft scripts, CLI tools, and full project structures using modern industry conventions.

Design philosophy:

- Minimal command surface
- Strict defaults
- Standard directory layouts
- CI-ready scaffolding
- Production-oriented structure
- Deterministic output

# COMMANDS

## new

Create a new project from a language template.

### SYNOPSIS

smithers new  [type]  [OPTIONS]

### ARGUMENTS

**<language>**

Required. One of:

bash
python
rust

**[type]**

Optional. Defaults to `project`.

Supported values:

draft
cli
project

**<name>**

Required. Name of the project directory to create.

Must not already exist unless `--force` is specified.

### OPTIONS

**--git**

Initialize a new Git repository in the generated project.

---

**--license <type>**

Include a license file.

Supported values:

mit
apache
none

Default: none

---

**--ci**

Include a CI workflow configuration (GitHub Actions).

---

**--author <name>**

Override detected author name in generated files.

---

**--force**

Overwrite existing directory if it already exists.

Use with caution.

---

**--dry-run**

Preview files and directories that would be created without writing to disk.

### EXAMPLES

Create a Rust CLI tool:

smithers new rust cli myapp –git –license mit

Create a Python project:

smithers new python myproject

Create a Bash draft script:

smithers new bash draft myscript

---

## list

List available languages and project types.

### SYNOPSIS

smithers list

### DESCRIPTION

Displays supported languages and corresponding template types.

---

## doctor

Check system dependencies and environment readiness.

### SYNOPSIS

smithers doctor

### DESCRIPTION

Performs diagnostic checks for required tooling:

- Bash availability
- Python installation
- Rust installation
- Cargo availability
- Git availability

Reports missing or misconfigured dependencies.

---

# GLOBAL OPTIONS

These options apply to all commands.

---

**-h, --help**

Display help information.

---

**-V, --version**

Display smithers version.

---

# GENERATED TEMPLATE STRUCTURES

## Bash Templates

### draft

/
├── .sh
├── README.md
└── .editorconfig

Includes:

- strict mode (`set -euo pipefail`)
- main() entry point
- argument forwarding

---

### cli

/
├── bin/
├── lib/
├── test/
├── Makefile
├── README.md
└── .gitignore

Includes:

- structured CLI skeleton
- test scaffold
- lint-ready setup (shellcheck compatible)

---

## Python Templates

### draft

/
├── main.py
├── requirements.txt
└── README.md

---

### cli

/
├── pyproject.toml
├── src//
│   ├── init.py
│   ├── cli.py
│   └── main.py
├── tests/
├── README.md
├── .gitignore
└── .editorconfig

Features:

- src layout
- entry point defined in pyproject
- pytest-ready
- type hints enabled
- PEP 621 metadata structure

---

### project

Adds:

docs/
.pre-commit-config.yaml
.github/workflows/ci.yml

If `--ci` is specified, includes CI workflow.

---

## Rust Templates

### draft

/
├── Cargo.toml
└── src/main.rs

Generated using `cargo new` conventions.

---

### cli

/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli.rs
│   └── lib.rs
├── tests/
├── README.md
├── .gitignore
└── rustfmt.toml

Features:

- clap dependency
- modular CLI structure
- integration test scaffold
- clippy-compatible setup

---

### project

Adds:

examples/
benches/
.github/workflows/ci.yml

If `--ci` is specified, includes CI workflow.

---

# EXIT STATUS

**0**  
Success.

**1**  
General error.

**2**  
Invalid arguments.

**3**  
Environment check failed.

---

# DESIGN PRINCIPLES

- No interactive prompts.
- Deterministic file generation.
- No implicit global configuration.
- Templates follow language community standards.
- Explicit over magical behavior.
- Safe by default (no overwrite without `--force`).

---

# FILES

No global configuration file is required.

Future versions may support:

~/.config/smithers/config.toml

---

# ENVIRONMENT

**SMITHERS_AUTHOR**

If set, used as default author value.

---

# SECURITY

smithers does not execute arbitrary template code.

It only writes static files and initializes repositories when requested.

---

# SEE ALSO

git(1), cargo(1), python(1), bash(1)

---

smithers 1.0.0  
Minimal. Opinionated. Industrial.


⸻

To Generate Man Page

pandoc smithers.md -s -t man -o smithers.1

To Generate Info Page

pandoc smithers.md -s -t info -o smithers.info