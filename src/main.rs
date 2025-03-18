use std::io;

fn main() {
    println!("Welcome to Task tracker");
    println!("-----------------------");
    loop {
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read Stdin");

        let commands = input.split(" ").collect::<Vec<&str>>();

        handle_command(commands);
        println!("-----------------------");
        println!("Ready for new command 😎");
    }
}

fn handle_command(commands: Vec<&str>) {
    match commands[0].trim().to_lowercase().as_str() {
        "add" => {
            handle_add_command(commands);
        }
        "update" => {
            handle_update_command(commands);
        }
        "delete" => {
            handle_delete_command(commands);
        }
        "list" => {
            handle_list_command(commands);
        }
        "mark-in-progress" | "mark-done" => {
            handle_mark_command(commands);
        }
        _ => {
            println!("Invalid command");
        }
    }
}

fn handle_mark_command(commands: Vec<&str>) {
    match commands[0].trim().to_lowercase().as_str() {
        "mark-in-progress" => {
            mark_in_progress();
        }
        "mark-done" => {
            mark_done();
        }
        _ => {
            println!("Invalid command");
        }
    }
}

fn handle_delete_command(commands: Vec<&str>) {
    delete();
}

fn handle_update_command(commands: Vec<&str>) {
    update();
}

fn handle_add_command(commands: Vec<&str>) {
    add();
}

fn handle_list_command(commands: Vec<&str>) {
    match commands
        .get(1)
        .map(|s| s.trim().to_lowercase().as_str().to_owned())
        .as_deref()
    {
        Some("") | None => list_all(),
        Some("done") => list_all_done(),
        Some("todo") => list_all_not_done(),
        Some("in-progress") => list_all_in_progress(),
        _ => {
            println!("Invalid command");
        }
    }
}

fn list_all() {
    println!("List all called!");
}

fn list_all_done() {
    println!("List done called!");
}

fn list_all_not_done() {
    println!("List not done called!");
}

fn list_all_in_progress() {
    println!("List in progress called!");
}

fn add() {
    println!("add called!");
}

fn mark_done() {
    println!("mark done called!");
}

fn mark_in_progress() {
    println!("mark in progress called!");
}

fn update() {
    println!("update called!");
}

fn delete() {
    println!("delete called!");
}
