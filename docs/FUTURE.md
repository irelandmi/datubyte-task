# Future Aspirations

## Vision

datubyte-task evolves from a coding task tracker into a **task orchestration layer** — the central nervous system that any AI agent or service can pull work from and report results to.

Use cases extend beyond code: classification, document processing, extraction, summarization, review, and anything else an agent can do.

## What datubyte-task is today

A task tracker with projects, epics, tasks, dependencies, statuses, activity logs, and SSE live updates. Shaped around a single agent (Claude Code) doing coding work via CLI and HTTP API.

## Key shifts

### 1. Agent-agnostic work items

Tasks should support arbitrary work types, not just coding. This means:

- A `task_type` or `kind` field on tasks
- Flexible structured `input` / `output` payloads (JSON) rather than assuming code artifacts
- Agent capability matching — "this task needs an agent that can do OCR"

### 2. Multiple concurrent agents

Move from one agent picking tasks sequentially to a pool of agents (possibly different models/tools) claiming work concurrently.

- **Task assignment / locking** — agent claims a task, heartbeats, timeout/reassign on failure
- **Agent registry** — what agents exist, what capabilities they have, their current load
- **Concurrency controls** — max parallel tasks per agent, rate limits

### 3. Workflow / pipeline support

Real work is rarely one task. Document processing is: ingest → classify → extract → validate → store.

- **Task chaining** beyond simple dependencies — outputs of one task become inputs to the next
- **Pipeline templates** — define a reusable workflow, instantiate it per document/batch
- **Fan-out / fan-in** — one classification task spawns N extraction tasks, then a merge step

### 4. The ecosystem

```
┌─────────────┐     ┌──────────────────┐     ┌─────────────┐
│  Ingestion  │────>│  datubyte-task    │<────│  Agent Pool  │
│  (files,    │     │  (orchestrator)   │────>│  (workers)   │
│   APIs,     │     │                   │     │              │
│   webhooks) │     │  - task queue     │     │  - coding    │
└─────────────┘     │  - dependencies   │     │  - classify  │
                    │  - routing        │     │  - extract   │
┌─────────────┐     │  - status/events  │     │  - review    │
│  Dashboard  │<────│  - results store  │     └─────────────┘
│  / API      │     └──────────────────┘
└─────────────┘            │
                    ┌──────┴──────┐
                    │  Results /  │
                    │  Artifacts  │
                    └─────────────┘
```

### 5. Events and integrations

- **Webhooks / event hooks** — external systems trigger pipelines and react to completions
- **SSE / streaming** — already exists, extend for multi-agent status feeds

## Priorities

What to build first:

1. **Structured input/output on tasks** — unlocks non-coding use cases without breaking what works
2. **Task claiming protocol** — agent registers, claims next matching task, posts result. This is the API contract other agents build against
3. **Webhooks or event hooks** — so external systems can trigger pipelines and react to completions

## What to defer

- **Don't build an agent runtime** — let agents be external processes that speak the API
- **Don't build a scheduler** — dependencies + a simple queue get you far
- **Don't over-specify task types upfront** — keep payloads as JSON blobs and let conventions emerge

## Design principle

datubyte-task should be a **dumb but reliable task broker** with a good API, not a smart orchestrator. Keep the intelligence in the agents and the workflow definitions, keep the infrastructure simple.
