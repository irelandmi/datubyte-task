# datubyte-task

Lightweight project management tool built for AI agent workflows. SQLite backend, Rust CLI and API server, terminal-themed web UI with live SSE updates.

## Prerequisites

- [Rust](https://rustup.rs/) (stable)
- [Node.js](https://nodejs.org/) (for the web UI)

## Install

```bash
git clone git@github.com:irelandmi/datubyte-task.git
cd datubyte-task

# Install the CLI and server binaries
cargo install --path crates/cli
cargo install --path crates/server

# Build the frontend
cd frontend && npm install && npm run build && cd ..
```

This installs `datubyte-task` and `datubyte-task-server` to `~/.cargo/bin/`. No external dependencies — SQLite is compiled from source.

## Quick Start

```bash
# Start the server with the web UI
datubyte-task-server --port 3000 --static-dir frontend/dist

# Use the CLI
datubyte-task project create "My Project"
datubyte-task task create --project <id> "My first task"
datubyte-task task list --project <id>
```

## Architecture

```
crates/
  core/       # SQLite schema, models, queries (datubyte-task-core)
  cli/        # CLI binary (datubyte-task)
  server/     # Axum API server (datubyte-task-server)
frontend/     # TypeScript + Vite web UI
docs/         # Architecture and schema docs
tests/        # E2E shell tests
```

## CLI

```bash
# Projects
datubyte-task project list
datubyte-task project create <name> [--description <desc>]
datubyte-task project show <id>
datubyte-task project delete <id>

# Epics
datubyte-task epic list --project <id>
datubyte-task epic create --project <id> <name> [--description <desc>]
datubyte-task epic show <id>
datubyte-task epic close <id>
datubyte-task epic delete <id>

# Tasks
datubyte-task task list --project <id> [--status <s>] [--epic <id>] [--assignee <a>] [--label <l>] [--kind <k>]
datubyte-task task create --project <id> <title> [--epic <id>] [--kind <k>] [--parent <id>] [--priority <p>] [--assignee <a>] [--label <l>]...
datubyte-task task show <id>
datubyte-task task update <id> [--title <t>] [--description <d>] [--status <s>] [--priority <p>] [--assignee <a>] [--epic <id>] [--kind <k>]
datubyte-task task done <id>
datubyte-task task delete <id>

# Task outputs (file paths, commit SHAs, URLs, free text)
datubyte-task task output <id> --kind <kind> --ref <ref> [--label <label>]
datubyte-task task outputs <id>

# Task dependencies
datubyte-task task block <id> --by <dep_id>
datubyte-task task unblock <id> --from <dep_id>

# Labels
datubyte-task label list
datubyte-task label create <name> [--color <hex>]
datubyte-task label delete <id>
```

IDs are human-readable (`bold-fox-a3f1`) and support prefix lookup (`bold-fox`).

## API

The server exposes a REST API at `/api`. All endpoints return JSON.

| Method | Path | Description |
|--------|------|-------------|
| GET | /api/projects | List projects |
| POST | /api/projects | Create project |
| GET | /api/projects/:id | Get project |
| PATCH | /api/projects/:id | Update project |
| DELETE | /api/projects/:id | Delete project |
| GET | /api/projects/:pid/epics | List epics |
| POST | /api/projects/:pid/epics | Create epic |
| GET | /api/epics/:id | Get epic |
| PATCH | /api/epics/:id | Update epic |
| DELETE | /api/epics/:id | Delete epic |
| GET | /api/projects/:pid/tasks | List tasks (with query filters) |
| POST | /api/projects/:pid/tasks | Create task |
| GET | /api/tasks/:id | Get task (includes children, outputs, dependencies) |
| PATCH | /api/tasks/:id | Update task |
| DELETE | /api/tasks/:id | Delete task |
| PUT | /api/tasks/:id/labels | Set task labels |
| GET | /api/tasks/:id/outputs | List task outputs |
| POST | /api/tasks/:id/outputs | Add task output |
| POST | /api/tasks/:id/dependencies | Add dependency |
| DELETE | /api/tasks/:id/dependencies/:dep_id | Remove dependency |
| GET | /api/tasks/:id/events | List activity events |
| POST | /api/tasks/:id/events | Add comment |
| GET | /api/labels | List labels |
| POST | /api/labels | Create label |
| DELETE | /api/labels/:id | Delete label |
| GET | /api/events | SSE event stream |

## Data Model

- **Projects** contain **epics** (every project gets a Backlog epic automatically)
- **Epics** contain **tasks** (tasks default to Backlog when no epic is specified)
- **Tasks** have a type (`story`, `task`, `spike`, `bug`, `chore`), status (`todo`, `in_progress`, `done`, `cancelled`, `blocked`), priority, assignee, and labels
- **Tasks** can have sub-tasks (one level of nesting via `parent_id`)
- **Task outputs** are lightweight references to artifacts: file paths, commit SHAs, URLs, or free text
- **Task dependencies** form a directed acyclic graph — circular dependencies are rejected at insert time

## Web UI

Terminal-themed kanban board with:

- Kanban and timeline views per project
- Five status columns: To Do, In Progress, Done, Cancelled, Blocked
- Task detail page with inline editing, sub-task creation, outputs, dependencies, and activity log
- Live updates via Server-Sent Events

## Testing

```bash
# Unit tests (21 tests)
cargo test --workspace

# E2E CLI tests (35 tests)
bash tests/cli_e2e.sh
```

## Docs

- [Database schema](docs/database.md) - tables, indexes, migrations, cascade behavior
- [Agent architecture](docs/agent-architecture.md) - planner/executor/coordinator roles
- [Open questions](docs/open-questions.md) - unresolved design decisions
