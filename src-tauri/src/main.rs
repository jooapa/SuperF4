// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread::sleep, time::Duration};

use inputbot::{KeybdKey::*, MouseButton::*};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command


#[tauri::command]
fn cps(name: &str) -> String {
    format!("1 Click per {}ms", name)
}

#[tauri::command]
fn startClicker() {
    CapsLockKey.bind(move || {
        while CapsLockKey.is_toggled() {
            LeftButton.press();
            LeftButton.release();

            sleep(Duration::from_millis(30));
        }
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}



fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cps])
        .invoke_handler(tauri::generate_handler![startClicker])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");    
}
