use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{self, Error, ErrorKind, prelude::*};
use std::time::{self, SystemTime};

use task::Task;
use task_service::TaskService;

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
    match TaskService::list(None) {
        Ok(task_list) => {
            dbg!(task_list);
        },
        Err(error) => {
            println!("{error}");
        },
    }
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
    match TaskService::add(description) {
        Ok(_) => {
            println!("Task added!");
        },
        Err(error) => {
            println!("{error}");
        },
    }
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
