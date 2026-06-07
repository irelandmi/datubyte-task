Pick up the next available task from a taskd project and start working on it.

## Input

$ARGUMENTS

## Instructions

1. If no project ID is given, run `taskd project list` and ask which project
2. Run `taskd task list --project <id> --status todo` to find available tasks
3. Check dependencies — run `taskd task show <id>` on candidates to find one with no unfinished dependencies
4. Select the highest priority ready task
5. Claim it: `taskd task update <id> --status in_progress --assignee claude`
6. Log the start: `taskd task log <id> "Starting work"`
7. Show the task details and begin working on it
8. When done, mark complete with `taskd task done <id>` and attach any outputs with `taskd task output <id> --kind <file|commit|url|text> --ref <reference>`
