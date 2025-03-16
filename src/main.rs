use serde::{self, Deserialize, Serialize};
use serde_json;
use std::{env, fs};

#[derive(Deserialize, Serialize)]
struct Task {
    name: String,
    done: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut tasks: Vec<Task> = Vec::new();

    if args.len() < 2 {
        println!("Usage: toro <action> <taskname> or \"<taskname with spaces>\"");
        return;
    }

    if !fs::metadata("tasks.json").is_ok() || fs::read_to_string("tasks.json").unwrap().is_empty() {
        fs::write("tasks.json", "[]").unwrap();
    } else {
        tasks = serde_json::from_str(&String::from_utf8(fs::read("tasks.json").unwrap()).unwrap()).expect("Couldn't parse tasks");
    }
    match args[1].as_str() {
        "add" => {
            tasks.push(Task {
                name: args[2].replace("\"", "").clone(),
                done: false,
            });
            fs::write("tasks.json", serde_json::to_string(&tasks).unwrap()).unwrap();
        },
        "d" => {
            for task in &mut tasks {
                if task.name == args[2].replace("\"", "").clone() {
                    task.done = true;
                    println!("Set {} to done", task.name)
                }
            }
            fs::write("tasks.json", serde_json::to_string(&tasks).unwrap()).unwrap();
        },
        "nd" => {
            for task in &mut tasks {
                if task.name == args[2].replace("\"", "").clone() {
                    task.done = false;
                    println!("Set {} to not done", task.name)
                }
            }
            fs::write("tasks.json", serde_json::to_string(&tasks).unwrap()).unwrap();
        }
        "rm" => {
            tasks.retain(|task| task.name != args[2]);
            fs::write("tasks.json", serde_json::to_string(&tasks).unwrap()).unwrap();
        }
        "show" => {
            println!("Tasks: ");
            for task in tasks {
                println!("{} - {}", task.name, if task.done {"✅"} else {"❌"})
            }
        },
        _ => {
            println!("Unrecognized command");
        }
    }
}
