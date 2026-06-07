Read the provided requirements document or description and decompose it into a taskd project structure.

## Input

$ARGUMENTS

## Instructions

1. Create a project with `taskd project create "<name>"`
2. Identify logical feature areas and create epics with `taskd epic create --project <id> "<name>"`
3. Within each epic, create stories (`--type story`), spikes (`--type spike`) for unknowns, bugs (`--type bug`), and tasks (`--type task`) using `taskd task create --project <id> --epic <id> "<title>" --type <type> --priority <priority>`
4. Break stories into concrete sub-tasks using `--parent <task_id>`
5. Set dependencies between tasks using `taskd task block <id> --by <dep_id>`
6. Set appropriate priorities: low, medium, high, urgent
7. Add labels for cross-cutting concerns using `taskd label create "<name>"` and `--label <name>` on task creation

## Output

After planning, run `taskd task list --project <id>` and summarize the created project structure.
