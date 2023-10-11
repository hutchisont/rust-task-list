use std::fmt::Formatter;

enum Status {
    Completed,
    NotCompleted,
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Status::Completed => write!(f, "[X]"),
            Status::NotCompleted => write!(f, "[ ]"),
        }
    }
}

struct Task {
    status: Status,
    description: String,
}

impl std::fmt::Display for Task {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status, self.description)
    }
}

pub struct TaskHandler {
    tasks: Vec<Task>,
}

impl TaskHandler {
    pub fn new() -> TaskHandler {
        return TaskHandler {
            tasks: Vec::new(),
        };
    }

    pub fn view_tasks(&self) -> String {
        let mut output;
        if self.tasks.len() == 0 {
            output = String::from("No tasks to view.")
        } else {
            output = String::from("Tasks:");
            for (index, task) in self.tasks.iter().enumerate() {
                output = format!("{}\n{}. {}", output, index + 1, task);
            }
        }
        return output;
    }

    pub fn mark_task_completed(&mut self, index: usize) -> String {
        let internal_index = index - 1;
        if self.tasks.len() >= internal_index {
            self.tasks[internal_index].status = Status::Completed;
            return format!("Task \"{}\" marked as complete.", self.tasks[internal_index].description);
        } else {
            return format!("Failed to update task, invalid index");
        }
    }

    pub fn add_task(&mut self, description: &str) -> String {
        let task = Task {
            status: Status::NotCompleted,
            description: String::from(description),
        };
        self.tasks.push(task);

        return format!("\nTask \"{description}\" added.");
    }
}

