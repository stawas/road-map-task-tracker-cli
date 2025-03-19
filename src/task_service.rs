use core::task;
use std::{
    fs::{File, OpenOptions},
    io::{self, Error, ErrorKind, Read, Seek, Write},
    time::SystemTime,
};

use crate::task::Task;
use crate::task_status::TaskStatus;

pub struct TaskService {}

impl TaskService {
    fn task_file() -> Result<File, Error> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("tasks.json")
    }

    fn create_task_data_json(task_list: Vec<Task>) -> Result<String, Error> {
        let error_result = |error: Error| -> Result<String, Error> {
            return Err(Error::new(
                error.kind(),
                format!("Error creating task json: {}", error),
            ));
        };

        let task_list_json = match serde_json::to_string(&task_list) {
            Ok(json) => json,
            Err(error) => return error_result(Error::new(ErrorKind::Other, error)),
        };
        Ok(task_list_json)
    }

    fn get_id() -> Result<i32, io::Error> {
        let open_count_file_result = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open("count.txt")?;
        let mut file = open_count_file_result;
        let mut count_from_file = String::new();
        let mut id_number = match io::Read::read_to_string(&mut file, &mut count_from_file) {
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

    pub fn add(description: String) -> Result<(), Error> {
        let error_result = |error| -> Result<(), Error> {
            return Err(Error::new(
                ErrorKind::Other,
                format!("Error adding task: {}", error),
            ));
        };

        let id = Self::get_id()?;

        let current_time_millis = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(current_time) => current_time.as_millis().to_string(),
            Err(error) => return error_result(Error::new(ErrorKind::Other, error)),
        };

        let mut task_list: Vec<Task> = Vec::new();

        task_list.push(Task {
            id: id,
            description: description,
            status: String::from("todo"),
            created_at: current_time_millis.clone(),
            updated_at: current_time_millis.clone(),
        });

        let task_json = Self::create_task_data_json(task_list)?;

        TaskService::task_file()?.write_all(format!("{}", task_json).as_bytes())?;

        Ok(())
    }

    pub fn list(status: Option<TaskStatus>) -> Result<Vec<Task>, Error> {
        let error_result = |error| -> Result<Vec<Task>, Error> {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Error list task {}: {}",
                    if status.is_some() {
                        status.unwrap().to_string() + "with status"
                    } else {
                        String::from("")
                    },
                    error
                ),
            ));
        };
        let mut task_list_json = String::new();
        TaskService::task_file()?.read_to_string(&mut task_list_json)?;

        let task_list: Vec<Task> = match serde_json::from_str(&task_list_json) {
            Ok(json) => json,
            Err(error) => return error_result(Error::new(ErrorKind::Other, error)),
        };

        match status {
            Some(value) => {
                Ok(task_list.into_iter().filter(|task_item| {
                    value.to_string() == task_item.status
                }).collect())
            },
            None => {
                Ok(task_list)
            },
        }
    }
}
