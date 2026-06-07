Show the current state of a taskd project board.

## Input

$ARGUMENTS

## Instructions

1. If no project ID is given, run `taskd project list` and ask which project to show
2. Run `taskd task list --project <id>` to get all tasks
3. Group and display tasks by status columns: **To Do**, **In Progress**, **Blocked**, **Done**, **Cancelled**
4. For each task show: ID, type, title, priority, assignee
5. Also run `taskd epic list --project <id>` and show epic status
6. Summarize: total tasks, counts per status, any blocked tasks and what they're blocked by
