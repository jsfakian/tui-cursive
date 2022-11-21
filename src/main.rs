use std::env;
use serde_json::{Value, json};

mod installer;
mod actions;
mod state;
mod views;
mod utils;
mod data;
mod error;

fn help() {
    println!("usage: tui-cursive <installer.json>
    input file <installer.json> is optional.");
}

fn main() {
    let mut installer_json = json!(null);
    let args: Vec<String> = env::args().collect();

    match args.len() {
        // no arguments passed
        1 => {
            println!("Interactive installer mode!");
        },
        2 => {
            installer_json = {
                let text = std::fs::read_to_string(&args[1]).unwrap();
                // Parse the string into a dynamically-typed JSON structure.
                serde_json::from_str::<Value>(&text).unwrap()
            }
        }
        // all the other cases
        _ => {
            // show a help message
            help();
        }
    }

    installer::run(installer_json)
}