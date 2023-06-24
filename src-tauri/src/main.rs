// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


#[tauri::command]
fn cps(name: &str) -> String {
    format!("1 Click per {}ms", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![cps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");    
}

fn processF4() {
    println!("F4 pressed");
}

#[tauri::command(rename_all = "snake_case")]
fn my_custom_command(invoke_message: String) {
  println!("I was invoked from JS, with this message: {}", invoke_message);
}