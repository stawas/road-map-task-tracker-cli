
use std::io;
use task_service::TaskService;
use task_status::TaskStatus;

pub mod task;
pub mod task_service;
pub mod task_status;

fn main() {
    println!("Welcome to Task tracker");
    println!("-----------------------");
    loop {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read Stdin");

        handle_command(input.trim().to_owned());
        println!("-----------------------");
        println!("Ready for new command 😎");
    }
}

fn handle_command(input: String) {
    let commands = input.split(" ").collect::<Vec<&str>>();
    match commands[0].trim().to_lowercase().as_str() {
        "add" => {
            handle_add_command(input);
        }
        "update" => {
            handle_update_command(input);
        }
        "delete" => {
            handle_delete_command(commands);
        }
        "list" => {
            handle_list_command(commands);
        }
        "mark-in-progress" | "mark-done" | "mark-todo" => {
            handle_mark_command(commands);
        }
        _ => {
            println!("Invalid command");
        }
    }
}

fn handle_mark_command(commands: Vec<&str>) {
    let status = commands[0].trim().to_lowercase();
    let task_id = match commands.get(1) {
        Some(value) => value.to_string(),
        None => {
            println!("Task id not found.");
            return
        }
    };
    match status.as_str() {
        "mark-in-progress" => {
            mark_with_status(task_id, TaskStatus::InProgress);
        }
        "mark-done" => {
            mark_with_status(task_id, TaskStatus::Done);
        }
        "mark-todo" => {
            mark_with_status(task_id, TaskStatus::Todo);
        }
        _ => {
            println!("Invalid command");
        }
    }
}

fn handle_delete_command(commands: Vec<&str>) {
    let task_id = match commands.get(1) {
        Some(value) => value.to_string(),
        None => {
            println!("Task id not found.");
            return
        }
    };
    delete(task_id);
}

fn handle_update_command(input: String) {
    let task_id = match input.split(" ").collect::<Vec<&str>>().get(1) {
        Some(value) => value.to_string(),
        None => {
            println!("Task id not found.");
            return
        }
    };
    let mut allow_collect = false;
    let mut description = String::from("");
    for (i, character) in input.chars().enumerate() {
        let c = String::from(character);
        if allow_collect && i < (input.chars().count() - 1) {
            description += &c;
        }
        if c == "\"" {
            allow_collect = true;
        }
    }
    update(task_id, description);
}

fn handle_add_command(input: String) {
    let mut allow_collect = false;
    let mut description = String::from("");
    for (i, character) in input.chars().enumerate() {
        let c = String::from(character);
        if allow_collect && i < (input.chars().count() - 1) {
            description += &c;
        }
        if c == "\"" {
            allow_collect = true;
        }
    }
    add(description);
}

fn handle_list_command(commands: Vec<&str>) {
    match commands
        .get(1)
        .map(|s| s.trim().to_lowercase().as_str().to_owned())
        .as_deref()
    {
        Some("") | None => list_all(),
        Some("done") => list_all_done(),
        Some("todo") => list_all_todo(),
        Some("in-progress") => list_all_in_progress(),
        _ => {
            println!("Invalid command");
        }
    }
}

fn list_all() {
    match TaskService::list(None) {
        Ok(task_list) => {
            dbg!(task_list);
        }
        Err(error) => {
            println!("{error}");
        }
    }
}

fn list_all_done() {
    match TaskService::list(Some(TaskStatus::Done)) {
        Ok(task_list) => {
            dbg!(task_list);
        }
        Err(error) => {
            println!("{error}");
        }
    }
}

fn list_all_todo() {
    match TaskService::list(Some(TaskStatus::Todo)) {
        Ok(task_list) => {
            dbg!(task_list);
        }
        Err(error) => {
            println!("{error}");
        }
    }
}

fn list_all_in_progress() {
    match TaskService::list(Some(TaskStatus::InProgress)) {
        Ok(task_list) => {
            dbg!(task_list);
        }
        Err(error) => {
            println!("{error}");
        }
    }
}

fn add(description: String) {
    match TaskService::add(description) {
        Ok(_) => {
            println!("Task added!");
        }
        Err(error) => {
            println!("{error}");
        }
    }
}

fn mark_with_status(task_id: String, task_status: TaskStatus) {
    let task_id_number: i32 = match task_id.parse() {
        Ok(value) => value,
        Err(error) => {
            println!("{error}");
            return;
        }
    };
    match TaskService::update_status(task_id_number, task_status) {
        Ok(_) => {
            println!("Marked {} as {}", task_id_number, task_status.to_string());
        }
        Err(error) => {
            println!("{error}");
        }
    }
}

fn update(task_id: String, description: String) {
    let task_id_number: i32 = match task_id.parse() {
        Ok(value) => value,
        Err(error) => {
            println!("{error}");
            return;
        }
    };

    match TaskService::update_description(task_id_number, description) {
        Ok(_) => {
            println!("Task updated!");
        }
        Err(error) => {
            println!("{error}");
        }
    }
}

fn delete(task_id: String) {
    let task_id_number: i32 = match task_id.parse() {
        Ok(value) => value,
        Err(error) => {
            println!("{error}");
            return;
        }
    };
    match TaskService::delete(task_id_number) {
        Ok(_) => {
            println!("Task with id = {} deleted!", task_id);
        }
        Err(error) => {
            println!("{error}");
        }
    }
}
