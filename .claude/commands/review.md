Review project progress and handle coordination tasks.

## Input

$ARGUMENTS

## Instructions

1. If no project ID is given, run `datubyte-task project list` and ask which project
2. Run `datubyte-task task list --project <id>` and summarize progress by status
3. Check for **blocked** tasks: `datubyte-task task list --project <id> --status blocked`
   - For each blocked task, show what it's blocked by and suggest resolution
4. Check for completed **spikes**: `datubyte-task task list --project <id> --status done --type spike`
   - Show spike outputs with `datubyte-task task outputs <id>`
   - Suggest follow-up tasks based on spike findings
5. Check epic progress: `datubyte-task epic list --project <id>`
   - For each epic, show task completion ratio
6. Identify tasks that could be unblocked or reprioritized
7. Summarize overall project health and recommended next actions
