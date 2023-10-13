# Rust Task List  
Initially started off as the below "Programming Assignment", which was generated from chatGPT just to give me an idea to start with.
I've since started expanding beyond the assignment so don't treat what the assignment mentions as the full list of features.

## Basic Rust Programming Assignment

**Objective**

The objective of this assignment is to practice fundamental programming concepts in Rust.

**Instructions**

In this assignment, you will create a simple Rust program to manage a list of tasks. Your program should allow users to
add tasks, mark tasks as complete, view the list of tasks, and quit the program.

**Requirements**

1. Initialize an empty vector to store tasks.
2. Create a menu that displays the following options to the user:
    - Add a task
    - Mark a task as complete
    - View tasks
    - Quit
3. Implement functions for each option:
    - `add_task`: This function should prompt the user for a task description and add it to the task list.
    - `complete_task`: This function should prompt the user for the index of a task to mark as complete and update its
      status.
    - `view_tasks`: This function should display all tasks along with their completion status.
    - `quit`: This function should exit the program.
4. Use a loop to repeatedly display the menu and execute the chosen option until the user decides to quit.
5. Ensure that the program handles invalid inputs gracefully (e.g., entering non-integer values, selecting invalid
   options, etc.).

**Example Output**

```
Welcome to the Task Manager!

Menu:
1. Add a task
2. Mark a task as complete
3. View tasks
4. Quit

Please enter your choice: 1
Enter task description: Finish programming assignment

Task "Finish programming assignment" added.

Menu:
1. Add a task
2. Mark a task as complete
3. View tasks
4. Quit

Please enter your choice: 3
Tasks:
1. [ ] Finish programming assignment

Menu:
1. Add a task
2. Mark a task as complete
3. View tasks
4. Quit

Please enter your choice: 2
Enter the index of the task to mark as complete: 1

Task "Finish programming assignment" marked as complete.

Menu:
1. Add a task
2. Mark a task as complete
3. View tasks
4. Quit

Please enter your choice: 3
Tasks:
1. [X] Finish programming assignment

Menu:
1. Add a task
2. Mark a task as complete
3. View tasks
4. Quit

Please enter your choice: 4
Goodbye!
```
