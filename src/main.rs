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
        print_menu();
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


fn print_menu() {
    println!("\nMenu:");
    println!("1. Add a task");
    println!("2. Mark a task as complete");
    println!("3. View tasks");
    println!("4. Quit\n");
}

fn get_next_command() -> Option<Command> {
    let prompt = "Please enter your choice: ";
    let choice = prompt_get_value(prompt);

    return Command::from_str(&choice);
}

fn handle_completed_task(handler: &mut task_handler::TaskHandler) {
    let prompt = "Enter the index of the task to mark as complete: ";
    let index = prompt_get_value(prompt);
    let index: usize = index.parse().expect("Failed to convert to int");

    handler.mark_task_completed(index);
}

fn handle_add_task(handler: &mut task_handler::TaskHandler) {
    let prompt = "Enter task description: ";
    let description = prompt_get_value(prompt);

    handler.add_task(&description);
}

fn prompt_get_value(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().ok();

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("Failed to read input");

    return line.trim().to_string();
}
