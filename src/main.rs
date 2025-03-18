use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, Error, ErrorKind, prelude::*};
use std::time::{self, SystemTime};

use crate::task::Task;
pub mod task;

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

fn handle_add_command(input: String) {
    let mut allowCollect = false;
    let mut description = String::from("");
    for (i, character) in input.chars().enumerate() {
        let c = String::from(character);
        if allowCollect && i < (input.chars().count() - 1) {
            description += &c;
        }
        if c == "\"" {
            allowCollect = true;
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

fn add(description: String) {
    let error_result = || -> (){
        println!("Error creating task.");
        return
    };

    let result = create_task_data(description);
    if result.is_err() {
        error_result();
        return
    }
    let task_file_result = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .create(true)
        .open("tasks.json");

    if task_file_result.is_err() {
        error_result();
        return
    }

    task_file_result.unwrap().write_all(format!("{}", result.unwrap()).as_bytes()).unwrap_or_else(|_| {
        error_result();
        return
    });
    println!("add called!");
}
fn create_task_data(description: String) -> Result<String, Error> {
    let error_result = |error: Error| -> Result<String, Error> {
        return Err(Error::new(
            error.kind(),
            format!("Error add task: {}", error),
        ));
    };
    let id = match get_id() {
        Ok(id) => id,
        Err(error) => return error_result(error),
    };

    let current_time_millis = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(current_time) => current_time.as_millis().to_string(),
        Err(error) => return error_result(Error::new(ErrorKind::Other, error)),
    };
    let task = Task {
        id: id,
        description: description,
        status: String::from("todo"),
        created_at: current_time_millis.clone(),
        updated_at: current_time_millis.clone(),
    };
    let task_json = match serde_json::to_string(&task) {
        Ok(json) => json,
        Err(error) => return error_result(Error::new(ErrorKind::Other, error)),
    };
    Ok(task_json)
}

fn get_id() -> Result<i32, io::Error> {
    let open_count_file_result = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("count.txt")?;
    let mut file = open_count_file_result;
    let mut count_from_file = String::new();
    let mut id_number = match file.read_to_string(&mut count_from_file) {
        Ok(_) => {
            let mut count = count_from_file.parse::<i32>().unwrap_or_else(|_| 0);
            count += 1;
            println!("{count}");
            count
        }
        Err(_) => 0,
    };
    file.set_len(0)?;
    file.rewind()?;
    file.write_all(format!("{}", id_number).as_bytes())?;
    file.rewind()?;
    count_from_file = String::new();
    id_number = match file.read_to_string(&mut count_from_file) {
        Ok(_) => match count_from_file.parse::<i32>() {
            Ok(n) => n,
            Err(error) => return Err(Error::new(ErrorKind::Other, error)),
        },
        Err(error) => return Err(error),
    };
    Ok(id_number)
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
