// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread::sleep, time::Duration};

use inputbot::{KeybdKey::*, MouseButton::*};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


#[tauri::command]
fn cps(name: &str) -> String {
    let formatted = format!("1 Click per {}ms", name);
    println!("{}", name);
    formatted // Return the formatted string
}

#[tauri::command]
fn startStopClicker(cpms: i32) {
    println!("Start Clicker: {}ms", cpms);
    // Rest of your code...
}

fn main() {
    let mut cpms: i32 = 0;
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cps])
        .invoke_handler(tauri::generate_handler![startStopClicker])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
