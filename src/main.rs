use std::io::Write;

enum Command {
    Add,
    MarkCompleted,
    View,
    Quit,
}

impl Command {
    fn from_str(s: &str) -> Option<Command> {
        match s {
            "1" => Some(Command::Add),
            "2" => Some(Command::MarkCompleted),
            "3" => Some(Command::View),
            "4" => Some(Command::Quit),
            _ => None,
        }
    }
}

enum Status {
    Completed,
    NotCompleted,
}

impl Status {
    fn to_str(&self) -> &str {
        match self {
            Status::Completed => "X",
            Status::NotCompleted => " "
        }
    }
}

struct Task {
    status: Status,
    description: String,
}

struct TaskHandler {
    tasks: Vec<Task>,
}

impl TaskHandler {
    fn print_tasks(&self) {
        if self.tasks.len() == 0 {
            println!("No tasks to view.")
        } else {
            println!("Tasks:");
            for (index, task) in self.tasks.iter().enumerate() {
                println!("{}. [{}] {}", index + 1, task.status.to_str(), task.description);
            }
        }
    }

    fn mark_task_completed(&mut self, index: usize) {
        println!();
        let internal_index = index - 1;
        if self.tasks.len() >= internal_index {
            self.tasks[internal_index].status = Status::Completed;
            println!("Task \"{}\" marked as complete.", self.tasks[internal_index].description);
        } else {
            println!("Failed to update task, invalid index");
        }
    }

    fn add_task(&mut self, description: &str) {
        let task = Task {
            status: Status::NotCompleted,
            description: String::from(description),
        };
        self.tasks.push(task);

        println!("\nTask \"{description}\" added.");
    }
}

fn main() {
    let mut handler = TaskHandler {
        tasks: Vec::new()
    };

    'main: loop {
        let choice = get_next_command();
        let command = match choice {
            None => {
                println!("Invalid input, please try again.");
                continue;
            }
            Some(option) => option
        };
        match command {
            Command::Add => handle_add_task(&mut handler),
            Command::MarkCompleted => handle_completed_task(&mut handler),
            Command::View => handler.print_tasks(),
            Command::Quit => break 'main
        }
    }

    println!("Goodbye!");
}


fn display_menu() {
    println!("\nMenu:");
    println!("1. Add a task");
    println!("2. Mark a task as complete");
    println!("3. View tasks");
    println!("4. Quit\n");
}

fn get_next_command() -> Option<Command> {
    display_menu();
    print!("Please enter your choice: ");
    std::io::stdout().flush().ok();

    let mut choice = String::new();
    std::io::stdin().read_line(&mut choice).expect("Failed to read input");
    return Command::from_str(&choice.trim());
}

fn handle_completed_task(handler: &mut TaskHandler) {
    print!("Enter the index of the task to mark as complete: ");
    std::io::stdout().flush().ok();

    let mut index = String::new();
    std::io::stdin().read_line(&mut index).expect("Failed to read input");
    let index: usize = index.trim().parse().expect("Failed to convert to int");

    handler.mark_task_completed(index);
}

fn handle_add_task(handler: &mut TaskHandler) {
    print!("Enter task description: ");
    std::io::stdout().flush().ok();

    let mut description = String::new();
    std::io::stdin().read_line(&mut description).expect("Failed to read input");

    handler.add_task(&description.trim());
}
