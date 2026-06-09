Pick up the next available task from a datubyte-task project and start working on it.

## Input

$ARGUMENTS

## Instructions

1. If no project ID is given, run `datubyte-task project list` and ask which project
2. Run `datubyte-task task list --project <id> --status todo` to find available tasks
3. Check dependencies — run `datubyte-task task show <id>` on candidates to find one with no unfinished dependencies
4. Select the highest priority ready task
5. Claim it: `datubyte-task task update <id> --status in_progress --assignee claude`
6. Log the start: `datubyte-task task log <id> "Starting work"`
7. Show the task details and begin working on it
8. When done, mark complete with `datubyte-task task done <id>` and attach any outputs with `datubyte-task task output <id> --kind <file|commit|url|text> --ref <reference>`
