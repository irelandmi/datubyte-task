Show detailed information about a taskd task.

## Input

$ARGUMENTS

## Instructions

1. Run `taskd task show <id>` using the provided task ID (supports prefix lookup)
2. If the task has children, show them grouped by status
3. If the task has dependencies, show their status to indicate whether this task is ready
4. If the task has outputs, list them
5. Run `taskd task events <id>` to show the activity log
