use std::io::Write;

mod task_handler;

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

fn main() {
    let mut handler = task_handler::TaskHandler::new();

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

    print!("Enter the index of the task to mark as complete: ");
    std::io::stdout().flush().ok();

    let mut index = String::new();
    std::io::stdin().read_line(&mut index).expect("Failed to read input");
    let index: usize = index.trim().parse().expect("Failed to convert to int");
fn handle_completed_task(handler: &mut task_handler::TaskHandler) {

    handler.mark_task_completed(index);
}

    print!("Enter task description: ");
fn handle_add_task(handler: &mut task_handler::TaskHandler) {
    std::io::stdout().flush().ok();

    let mut description = String::new();
    std::io::stdin().read_line(&mut description).expect("Failed to read input");

    handler.add_task(&description.trim());
}
