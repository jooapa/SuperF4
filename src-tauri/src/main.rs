#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs::File;
use std::io::{Error, Read, Write};
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Blacklist {
    blacklist: Vec<String>,
}

fn main() {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![remove_exe_from_json, add_exe_to_json])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");    
}

#[tauri::command]
fn add_exe_to_json(name: &str) {
    println!("{}", name);
    if let Err(err) = add_to_blacklist(name) {
        eprintln!("Error: {}", err);
    } else {
        println!("Added 'lol.exe' to the blacklist.");
    }
}

#[tauri::command]
fn remove_exe_from_json(name: &str) {
    println!("{}", name);
    if let Err(err) = remove_from_blacklist(name) {
        eprintln!("Error: {}", err);
    } else {
        println!("Removed 'lol.exe' from the blacklist.");
    }
}

fn add_to_blacklist(entry: &str) -> Result<(), Error> {
    // Get the path of the current executable
    let exe_path = env::current_exe()?;
    
    // Create a path for the blacklist.json in the same directory as the executable
    let mut blacklist_path = PathBuf::from(exe_path.parent().unwrap());
    blacklist_path.push("blacklist.json");

    // Read the existing JSON file
    let mut file = File::open(blacklist_path.clone())?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON into a Blacklist struct
    let mut blacklist: Blacklist = serde_json::from_str(&contents)?;

    // Add the entry to the blacklist
    blacklist.blacklist.push(entry.to_string());

    // Serialize the updated blacklist back to JSON
    let updated_contents = serde_json::to_string_pretty(&blacklist)?;

    // Write the updated JSON back to the file
    let mut file = File::create(blacklist_path)?;
    file.write_all(updated_contents.as_bytes())?;

    Ok(())
}

fn remove_from_blacklist(entry: &str) -> Result<(), Error> {
    // Get the path of the current executable
    let exe_path = env::current_exe()?;
    
    // Create a path for the blacklist.json in the same directory as the executable
    let mut blacklist_path = PathBuf::from(exe_path.parent().unwrap());
    blacklist_path.push("blacklist.json");

    // Read the existing JSON file
    let mut file = File::open(&blacklist_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON into a Blacklist struct
    let mut blacklist: Blacklist = serde_json::from_str(&contents)?;

    // Remove the entry from the blacklist
    let index = blacklist.blacklist.iter().position(|x| x == entry);
    if let Some(idx) = index {
        blacklist.blacklist.remove(idx);
    } else {
        println!("Entry '{}' not found in the blacklist.", entry);
        return Ok(());
    }

    // Serialize the updated blacklist back to JSON
    let updated_contents = serde_json::to_string_pretty(&blacklist)?;

    // Write the updated JSON back to the file
    let mut file = File::create(&blacklist_path)?;
    file.write_all(updated_contents.as_bytes())?;

    Ok(())
}