#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::env;
use std::fs::File;
use std::io::{Error, Read, Write};

use std::path::PathBuf;
use serde::{Deserialize, Serialize};

use tauri::{SystemTray, SystemTrayMenu, SystemTrayEvent};
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize)]
struct Blacklist {
    blacklist: Vec<String>,
}

fn main() {
    let tray_menu = SystemTrayMenu::new(); // insert the menu items here

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![remove_exe_from_json, add_exe_to_json, get_blacklist_name, close_app, start_f4])
    .system_tray(SystemTray::new().with_menu(tray_menu))
    .on_system_tray_event(|app, event| match event {
    SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
    } => {
        println!("system tray received a left click");
        //if window is hidden then show it otherwise hide it
        let window = app.get_window("main").unwrap();
        if window.is_visible().unwrap() {
            window.hide().unwrap();
        } else {
            window.show().unwrap();
            
        }
    }
    SystemTrayEvent::MenuItemClick { id, .. } => {
        match id.as_str() {
        "quit" => {
            std::process::exit(0);
        }
        "hide" => {
            let window = app.get_window("main").unwrap();
            window.hide().unwrap();
        }
        _ => {}
        }
    }
    _ => {}
    })
    .build(tauri::generate_context!())
    .expect("error while building tauri application")
    .run(|_app_handle, event| match event {
    tauri::RunEvent::ExitRequested { api, .. } => {
    api.prevent_exit();
    }
    _ => {}
    });
}

#[tauri::command]
fn add_exe_to_json(name: &str) {
    println!("{}", name);
    if let Err(err) = add_to_blacklist(name) {
        eprintln!("Error: {}", err);
    } else {
        println!("Added {} to the blacklist.", name);
    }
}

#[tauri::command]
fn remove_exe_from_json(name: &str) {
    println!("{}", name);
    if let Err(err) = remove_from_blacklist(name) {
        eprintln!("Error: {}", err);
    } else {
        println!("Removed {} from the blacklist.", name);
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

#[tauri::command]
fn get_blacklist_name() -> Option<Blacklist> {
    // Get the path of the current executable
    let exe_path = match env::current_exe() {
        Ok(path) => path,
        Err(e) => {
            eprintln!("Failed to get current executable path: {}", e);
            return None;
        }
    };

    // Create a path for the blacklist.json in the same directory as the executable
    let mut blacklist_path = PathBuf::from(exe_path.parent().unwrap());
    blacklist_path.push("blacklist.json");

    let file = match std::fs::File::open(&blacklist_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file {}: {}", blacklist_path.display(), e);
            return None;
        }
    };

    let reader = std::io::BufReader::new(file);

    let config: Blacklist = match serde_json::from_reader(reader) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to deserialize blacklist: {}", e);
            return None;
        }
    };

    Some(config)
}

#[tauri::command]
fn close_app(app: tauri::AppHandle) {
    let window = app.get_window("main").unwrap();
    window.hide().unwrap();
}

//-----------------------------------------------------

use std::cell::RefCell;
use std::sync::Mutex;
use std::process::Command;
use inputbot::{KeybdKey::*};
use std::io::BufReader;

//get exe
use std::os::windows::ffi::OsStringExt;
use std::ffi::OsString;
use std::path::Path;
use winapi::um::winuser::{GetForegroundWindow, GetWindowThreadProcessId};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::GetModuleFileNameExW;
use winapi::shared::minwindef::{DWORD, MAX_PATH};

#[tauri::command]
fn start_f4() {
    let code_executed = Mutex::new(RefCell::new(false));

    RControlKey .bind(move || {
        // This code will be executed when Scroll Lock is pressed <held down>
        while RControlKey .is_pressed() {
            if F11Key.is_pressed() && !*code_executed.lock().unwrap().borrow() {
                let config = match get_blacklist_name() {
                    Some(value) => value,
                    None => return,
                };

                let exe_name = get_foreground_exe_name().unwrap();
                //taskkill program, if not in blacklist
                if config.blacklist.contains(&exe_name.to_string()) {
                    println!("blacklist: {}", exe_name);
                }
                else{
                    println!("exe_name: {}", exe_name);
                let output = Command::new("taskkill")
                    .args(&["/F", "/IM", &exe_name])
                    .output()
                    .expect("failed to execute process");
                if output.status.success() {
                    println!("Program terminated successfully!");
                } else {
                    println!("Failed to terminate program!");
                }
                
            }
            *code_executed.lock().unwrap().borrow_mut() = true;
            } else if F11Key.is_pressed() && *code_executed.lock().unwrap().borrow() {
                // F11 has already been pressed and code has already been executed,
                // so exit the loop early to avoid printing the message multiple times.
                break;
            }
            //ignore blacklist.json file, when pressed F10
            if F10Key.is_pressed() && !*code_executed.lock().unwrap().borrow() {
                let config = match get_blacklist_name() {
                    Some(value) => value,
                    None => return,
                };

                let exe_name = get_foreground_exe_name().unwrap();
                let output = Command::new("taskkill")
                    .args(&["/F", "/IM", &exe_name])
                    .output()
                    .expect("failed to execute process");
                if output.status.success() {
                    println!("Program terminated successfully!");
                } else {
                    println!("Failed to terminate program!");
                }
                *code_executed.lock().unwrap().borrow_mut() = true;
            }
            else if F10Key.is_pressed() && *code_executed.lock().unwrap().borrow() {
                // F10 has already been pressed and code has already been executed,
                // so exit the loop early to avoid printing the message multiple times.
                break;
            }
        }
        *code_executed.lock().unwrap().borrow_mut() = false; // Reset the flag when Scroll Lock is released
    });
    
    inputbot::handle_input_events();
    
}

fn get_foreground_exe_name() -> Option<String> {
    let hwnd = unsafe { GetForegroundWindow() };
    let mut pid = 0 as DWORD;
    unsafe { GetWindowThreadProcessId(hwnd, &mut pid) };
    if pid == 0 { return None; }
    
    let handle = unsafe { OpenProcess(0x0400 | 0x0010, 0, pid) };
    if handle.is_null() { return None; }
    
    let mut buffer = [0u16; MAX_PATH];
    let len = unsafe { GetModuleFileNameExW(handle, 0 as _, buffer.as_mut_ptr(), MAX_PATH as _) };
    if len == 0 { return None; }
    
    let exe_name = OsString::from_wide(&buffer[..len as usize]);
    let exe_path = Path::new(&exe_name);
    Some(exe_path.file_name()?.to_string_lossy().into_owned())
}
