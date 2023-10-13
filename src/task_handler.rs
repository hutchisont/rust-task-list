use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
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
        TaskHandler {
            tasks: Vec::new(),
        }
    }

    pub fn view_tasks(&self) -> String {
        let mut output;
        if self.tasks.is_empty() {
            output = String::from("No tasks to view.")
        } else {
            output = String::from("Tasks:");
            for (index, task) in self.tasks.iter().enumerate() {
                output = format!("{}\n{}. {}", output, index + 1, task);
            }
        }
        output
    }

    pub fn mark_task_completed(&mut self, index: usize) -> String {
        if index > 0 {
            if let Some(task) = self.tasks.get_mut(index - 1) {
                task.status = Status::Completed;
                return format!("Task \"{}\" marked as complete.", task.description);
            }
        }
        "Failed to update task, invalid index".to_string()
    }

    pub fn add_task(&mut self, description: &str) -> String {
        let task = Task {
            status: Status::NotCompleted,
            description: String::from(description),
        };
        self.tasks.push(task);

        format!("\nTask \"{description}\" added.")
    }
}

#[cfg(test)]
mod tests {
    use crate::task_handler::{Status, TaskHandler};

    #[test]
    fn when_task_added_collection_has_one_additional_task() {
        let mut handler = TaskHandler::new();

        assert!(handler.tasks.is_empty());

        handler.add_task("test_task");

        assert_eq!(1, handler.tasks.len());
    }

    #[test]
    fn when_task_marked_as_completed_task_status_is_set_to_completed() {
        let mut handler = TaskHandler::new();
        handler.add_task("test_task");

        assert_eq!(Status::NotCompleted, handler.tasks[0].status);

        handler.mark_task_completed(1);

        assert_eq!(Status::Completed, handler.tasks[0].status);
    }

    #[test]
    fn when_mark_ask_completed_given_invalid_index_handler_returns_error_message() {
        let mut handler = TaskHandler::new();

        let expected_error = "Failed to update task, invalid index";
        let error = handler.mark_task_completed(0);

        assert_eq!(error, expected_error);
    }
}
