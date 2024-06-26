use std::env;
use colored::Colorize;
use std::fs::{OpenOptions, File};
use serde::{Deserialize, Serialize};
use serde_json::Result;
use std::io::{prelude::*, read_to_string, stdin};

#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    info: String,
    completed: bool,
}

fn new_item(args: Vec<String>) -> Result<()> {
    println!("What is your new task called? ");
    let mut task_name = String::new();
    stdin().read_line(&mut task_name).expect("");
    
    println!("Describe the task: ");
    let mut details = String::new();
    stdin().read_line(&mut details).expect("");

    let new_task = Task {
        name: task_name.trim_end().to_string(),
        info: details.trim_end().to_string(),
        completed: false,
    };

    let json_task = serde_json::to_string(&new_task)?;
    let final_json = format!("{}\n", json_task);


    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("checklist.chk")
        .unwrap();

    file.write(final_json.as_bytes()).unwrap();
    println!("Your task was added.");

    Ok(())
}

fn new_checklist(args: Vec<String>) -> Result<()> {
    File::create_new("checklist.chk").unwrap();
    println!("Your checklist was created");

    Ok(())
}

fn complete_item(args: Vec<String>) -> Result<()> {
    let mut was_complete = false;
    let mut task_name_split = args.clone();
    task_name_split.drain(0..2);
    let task_name = task_name_split.join(" ");

    let raw_data = std::fs::read_to_string("checklist.chk").unwrap();
    let split_data = raw_data.split('\n').take_while(|x| x.len() > 0);
    let mut items: Vec<Task> = split_data
        .map(|x| serde_json::from_str(x).unwrap())
        .collect();

    let mut file_output = String::new();

    for tsk in items.iter_mut() {
        if tsk.name == task_name {
            tsk.completed = true;
            was_complete = true;
        }
        let task_json = serde_json::to_string(&tsk)?;
        file_output = format!("{}{}\n", file_output, task_json);
    }

    if !was_complete {
        println!("There was no task named: {}", task_name);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .open("checklist.chk")
        .unwrap();

    file.write_all(file_output.as_bytes()).unwrap();

    Ok(())
}

fn uncomplete_item(args: Vec<String>) -> Result<()> {
    let mut was_complete = false;
    let mut task_name_split = args.clone();
    task_name_split.drain(0..2);
    let task_name = task_name_split.join(" ");

    let raw_data = std::fs::read_to_string("checklist.chk").unwrap();
    let split_data = raw_data.split('\n').take_while(|x| x.len() > 0);
    let mut items: Vec<Task> = split_data
        .map(|x| serde_json::from_str(x).unwrap())
        .collect();

    let mut file_output = String::new();

    for tsk in items.iter_mut() {
        if tsk.name == task_name {
            tsk.completed = false;
            was_complete = true;
        }
        let task_json = serde_json::to_string(&tsk)?;
        file_output = format!("{}{}\n", file_output, task_json);
    }

    if !was_complete {
        println!("There was no task named: {}", task_name);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .open("checklist.chk")
        .unwrap();

    file.write_all(file_output.as_bytes()).unwrap();

    Ok(())
}

fn display_list(args: Vec<String>) -> Result<()> {
    let raw_data = std::fs::read_to_string("checklist.chk").unwrap();
    let split_data = raw_data.split('\n').take_while(|x| x.len() > 0);
    let mut items: Vec<Task> = split_data
        .map(|x| serde_json::from_str(x).unwrap())
        .collect();

    for tsk in items.iter_mut() {
        if tsk.completed {
            println!("====> {}",tsk.name.green());
        } else {
            println!("====> {}", tsk.name.red());
        }
    }

    Ok(())
}

fn remove_task(args: Vec<String>) -> Result<()> {
    let mut was_complete = false;
    let mut task_name_split = args.clone();
    task_name_split.drain(0..2);
    let task_name = task_name_split.join(" ");

    let raw_data = std::fs::read_to_string("checklist.chk").unwrap();
    let split_data = raw_data.split('\n').take_while(|x| x.len() > 0);
    let mut items: Vec<Task> = split_data
        .map(|x| serde_json::from_str(x).unwrap())
        .collect();

    let mut file_output = String::new();

    for tsk in items.iter_mut() {
        if tsk.name != task_name {
            let task_json = serde_json::to_string(&tsk)?;
            file_output = format!("{}{}\n", file_output, task_json);
        } else {
            was_complete = true;
        }
    }

    if !was_complete {
        println!("There was no task named: {}", task_name);
    }

    let mut file = OpenOptions::new()
        .write(true)
        .append(false)
        .open("checklist.chk")
        .unwrap();

    file.write_all(file_output.as_bytes()).unwrap();

    Ok(())
}

fn get_details(args: Vec<String>) -> Result<()> {
    let mut was_complete = false;
    let mut task_name_split = args.clone();
    task_name_split.drain(0..2);
    let task_name = task_name_split.join(" ");

    let raw_data = std::fs::read_to_string("checklist.chk").unwrap();
    let split_data = raw_data.split('\n').take_while(|x| x.len() > 0);
    let mut items: Vec<Task> = split_data
        .map(|x| serde_json::from_str(x).unwrap())
        .collect();


    for tsk in items.iter_mut() {
        if tsk.name == task_name {
            println!("{}:", tsk.name.bold().underline());
            println!("  >{}", tsk.info);
            was_complete = true;
        }
    }

    if !was_complete {
        println!("There was no task named: {}", task_name);
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    
    match command.as_str() {
        "new" => new_checklist(args),
        "add" => new_item(args),
        "check" | "c" => complete_item(args),
        "uncheck" | "u" => uncomplete_item(args),
        "list" | "l" => display_list(args),
        "remove" | "r" => remove_task(args),
        "details" | "d" => get_details(args),
        _ => {println!("There is not command: {command}");
        Ok(())},
    };
}
