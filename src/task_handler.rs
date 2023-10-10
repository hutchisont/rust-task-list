enum Status {
    Completed,
    NotCompleted,
}

impl Status {
    fn to_str(&self) -> &str {
        match self {
            Status::Completed => "X",
            Status::NotCompleted => " ",
        }
    }
}

struct Task {
    status: Status,
    description: String,
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

    pub fn print_tasks(&self) {
        if self.tasks.len() == 0 {
            println!("No tasks to view.")
        } else {
            println!("Tasks:");
            for (index, task) in self.tasks.iter().enumerate() {
                println!("{}. [{}] {}", index + 1, task.status.to_str(), task.description);
            }
        }
    }

    pub fn mark_task_completed(&mut self, index: usize) {
        println!();
        let internal_index = index - 1;
        if self.tasks.len() >= internal_index {
            self.tasks[internal_index].status = Status::Completed;
            println!("Task \"{}\" marked as complete.", self.tasks[internal_index].description);
        } else {
            println!("Failed to update task, invalid index");
        }
    }

    pub fn add_task(&mut self, description: &str) {
        let task = Task {
            status: Status::NotCompleted,
            description: String::from(description),
        };
        self.tasks.push(task);

        println!("\nTask \"{description}\" added.");
    }
}

