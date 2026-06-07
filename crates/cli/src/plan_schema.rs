use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Serialize, Deserialize)]
pub struct Plan {
	pub project_name: String,
	pub project_description: String,
	pub epics: Vec<PlannedEpic>,
	#[serde(default)]
	pub labels: Vec<PlannedLabel>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedEpic {
	pub temp_id: String,
	pub name: String,
	pub description: String,
	pub tasks: Vec<PlannedTask>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedTask {
	pub temp_id: String,
	pub title: String,
	#[serde(default)]
	pub description: String,
	#[serde(default = "default_kind")]
	pub kind: String,
	#[serde(default = "default_priority")]
	pub priority: String,
	pub parent: Option<String>,
	#[serde(default)]
	pub depends_on: Vec<String>,
	#[serde(default)]
	pub labels: Vec<String>,
}

fn default_kind() -> String {
	"task".to_string()
}

fn default_priority() -> String {
	"medium".to_string()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlannedLabel {
	pub name: String,
	#[serde(default = "default_color")]
	pub color: String,
}

fn default_color() -> String {
	"#6b7280".to_string()
}

const VALID_KINDS: &[&str] = &["story", "task", "spike", "bug", "chore"];
const VALID_PRIORITIES: &[&str] = &["low", "medium", "high", "urgent"];

pub fn validate(plan: &Plan) -> Result<(), Vec<String>> {
	let mut errors = Vec::new();

	if plan.project_name.is_empty() {
		errors.push("project_name is empty".to_string());
	}
	if plan.epics.is_empty() {
		errors.push("plan has no epics".to_string());
	}

	let mut all_task_ids = HashSet::new();
	let mut epic_ids = HashSet::new();

	for epic in &plan.epics {
		if !epic_ids.insert(&epic.temp_id) {
			errors.push(format!("duplicate epic temp_id: {}", epic.temp_id));
		}
		for task in &epic.tasks {
			if !all_task_ids.insert(&task.temp_id) {
				errors.push(format!("duplicate task temp_id: {}", task.temp_id));
			}
		}
	}

	let label_names: HashSet<&str> = plan.labels.iter().map(|l| l.name.as_str()).collect();

	for epic in &plan.epics {
		for task in &epic.tasks {
			if !VALID_KINDS.contains(&task.kind.as_str()) {
				errors.push(format!("{}: invalid kind '{}'", task.temp_id, task.kind));
			}
			if !VALID_PRIORITIES.contains(&task.priority.as_str()) {
				errors.push(format!("{}: invalid priority '{}'", task.temp_id, task.priority));
			}
			if let Some(ref parent) = task.parent {
				if !all_task_ids.contains(parent) {
					errors.push(format!("{}: parent '{}' not found", task.temp_id, parent));
				}
			}
			for dep in &task.depends_on {
				if !all_task_ids.contains(dep) {
					errors.push(format!("{}: dependency '{}' not found", task.temp_id, dep));
				}
				if dep == &task.temp_id {
					errors.push(format!("{}: depends on itself", task.temp_id));
				}
			}
			for label in &task.labels {
				if !label_names.contains(label.as_str()) {
					errors.push(format!("{}: label '{}' not defined in plan", task.temp_id, label));
				}
			}
		}
	}

	// Cycle detection on dependencies
	let mut adj: HashMap<&str, Vec<&str>> = HashMap::new();
	for epic in &plan.epics {
		for task in &epic.tasks {
			adj.entry(task.temp_id.as_str()).or_default();
			for dep in &task.depends_on {
				adj.entry(task.temp_id.as_str()).or_default().push(dep.as_str());
			}
		}
	}
	if has_cycle(&adj) {
		errors.push("dependency cycle detected".to_string());
	}

	if errors.is_empty() { Ok(()) } else { Err(errors) }
}

fn has_cycle(adj: &HashMap<&str, Vec<&str>>) -> bool {
	let mut visited = HashSet::new();
	let mut in_stack = HashSet::new();

	for &node in adj.keys() {
		if dfs_cycle(node, adj, &mut visited, &mut in_stack) {
			return true;
		}
	}
	false
}

fn dfs_cycle<'a>(
	node: &'a str,
	adj: &HashMap<&'a str, Vec<&'a str>>,
	visited: &mut HashSet<&'a str>,
	in_stack: &mut HashSet<&'a str>,
) -> bool {
	if in_stack.contains(node) {
		return true;
	}
	if visited.contains(node) {
		return false;
	}
	visited.insert(node);
	in_stack.insert(node);
	if let Some(deps) = adj.get(node) {
		for dep in deps {
			if dfs_cycle(dep, adj, visited, in_stack) {
				return true;
			}
		}
	}
	in_stack.remove(node);
	false
}

pub fn topological_order(plan: &Plan) -> Vec<&PlannedTask> {
	let mut tasks: Vec<&PlannedTask> = plan.epics.iter().flat_map(|e| &e.tasks).collect();
	let all_ids: HashSet<&str> = tasks.iter().map(|t| t.temp_id.as_str()).collect();

	// Kahn's algorithm
	let mut in_degree: HashMap<&str, usize> = HashMap::new();
	let mut dependents: HashMap<&str, Vec<&str>> = HashMap::new();
	for t in &tasks {
		in_degree.entry(t.temp_id.as_str()).or_insert(0);
		// parent counts as a dependency for ordering (parent must be created first)
		let mut deps: Vec<&str> = t.depends_on.iter().map(|s| s.as_str()).collect();
		if let Some(ref p) = t.parent {
			deps.push(p.as_str());
		}
		for dep in deps {
			if all_ids.contains(dep) {
				*in_degree.entry(t.temp_id.as_str()).or_insert(0) += 1;
				dependents.entry(dep).or_default().push(t.temp_id.as_str());
			}
		}
	}

	let mut queue: Vec<&str> = in_degree.iter()
		.filter(|(_, &deg)| deg == 0)
		.map(|(&id, _)| id)
		.collect();
	queue.sort(); // deterministic
	let mut order = Vec::new();

	while let Some(id) = queue.pop() {
		order.push(id);
		if let Some(deps) = dependents.get(id) {
			for &dep in deps {
				if let Some(deg) = in_degree.get_mut(dep) {
					*deg -= 1;
					if *deg == 0 {
						queue.push(dep);
						queue.sort();
					}
				}
			}
		}
	}

	// Reorder tasks by the topological order
	let pos: HashMap<&str, usize> = order.iter().enumerate().map(|(i, &id)| (id, i)).collect();
	tasks.sort_by_key(|t| pos.get(t.temp_id.as_str()).copied().unwrap_or(usize::MAX));
	tasks
}

pub fn preview(plan: &Plan) {
	let total_tasks: usize = plan.epics.iter().map(|e| e.tasks.len()).sum();
	let total_deps: usize = plan.epics.iter()
		.flat_map(|e| &e.tasks)
		.map(|t| t.depends_on.len())
		.sum();

	println!("Project: {}", plan.project_name);
	if !plan.project_description.is_empty() {
		println!("  {}", plan.project_description);
	}
	println!();
	println!("{} epics, {} tasks, {} dependencies, {} labels",
		plan.epics.len(), total_tasks, total_deps, plan.labels.len());
	println!();

	for epic in &plan.epics {
		println!("  [{}] {} ({} tasks)", epic.temp_id, epic.name, epic.tasks.len());
		for task in &epic.tasks {
			let mut meta = vec![task.kind.clone(), task.priority.clone()];
			if let Some(ref p) = task.parent {
				meta.push(format!("parent:{p}"));
			}
			if !task.depends_on.is_empty() {
				meta.push(format!("deps:{}", task.depends_on.join(",")));
			}
			if !task.labels.is_empty() {
				meta.push(format!("labels:{}", task.labels.join(",")));
			}
			println!("    [{}] {} ({})", task.temp_id, task.title, meta.join(", "));
		}
	}
}
