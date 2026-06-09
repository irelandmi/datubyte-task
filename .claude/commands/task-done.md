Mark a task as done and record its outputs.

## Input

$ARGUMENTS

## Instructions

1. Run `datubyte-task task show <id>` to verify the task exists and see its current state
2. If the task has children, check that all children are done or cancelled — warn if not
3. Attach any outputs from the work:
   - Files changed: `datubyte-task task output <id> --kind file --ref <path>`
   - Commits made: `datubyte-task task output <id> --kind commit --ref <sha>`
   - Relevant URLs: `datubyte-task task output <id> --kind url --ref <url>`
   - Notes/findings: `datubyte-task task output <id> --kind text --ref "<description>"`
4. Mark done: `datubyte-task task done <id>`
5. Log completion: `datubyte-task task log <id> "Completed: <brief summary>"`
