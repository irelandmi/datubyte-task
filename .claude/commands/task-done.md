Mark a task as done and record its outputs.

## Input

$ARGUMENTS

## Instructions

1. Run `taskd task show <id>` to verify the task exists and see its current state
2. If the task has children, check that all children are done or cancelled — warn if not
3. Attach any outputs from the work:
   - Files changed: `taskd task output <id> --kind file --ref <path>`
   - Commits made: `taskd task output <id> --kind commit --ref <sha>`
   - Relevant URLs: `taskd task output <id> --kind url --ref <url>`
   - Notes/findings: `taskd task output <id> --kind text --ref "<description>"`
4. Mark done: `taskd task done <id>`
5. Log completion: `taskd task log <id> "Completed: <brief summary>"`
