#![windows_subsystem = "windows"]
//detect keypresses
use inputbot::{KeybdKey::*};

use std::cell::RefCell;
use std::sync::Mutex;

use std::process::Command;

use std::fs::File;
use std::io::BufReader;
use serde::{Deserialize, Serialize};
//get exe
use std::os::windows::ffi::OsStringExt;
use std::ffi::OsString;
use std::path::Path;
use winapi::um::winuser::{GetForegroundWindow, GetWindowThreadProcessId};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::psapi::GetModuleFileNameExW;
use winapi::shared::minwindef::{DWORD, MAX_PATH};

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

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    blacklist: Vec<String>,
}

fn main() {
    //hide console
    use std::ptr;
    use winapi::um::wincon::GetConsoleWindow;
    use winapi::um::winuser::{ShowWindow, SW_HIDE};
    unsafe { winapi::um::wincon::FreeConsole() };

    let window = unsafe {GetConsoleWindow()};
    // https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-showwindow
    if window != ptr::null_mut() {
        unsafe {
            ShowWindow(window, SW_HIDE);
        }
    }

    let config = match getblacklistname() {
        Some(value) => value,
        None => return,
    };

    let code_executed = Mutex::new(RefCell::new(false));

    RControlKey .bind(move || {
        // This code will be executed when Scroll Lock is pressed <held down>
        while RControlKey .is_pressed() {
            if F11Key.is_pressed() && !*code_executed.lock().unwrap().borrow() {
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

fn getblacklistname() -> Option<Config> {
    let file_name = "blacklist.json";
    let file = match File::open(&file_name) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to open file {}: {}", file_name, e);
            return None;
        }
    };
    let reader = BufReader::new(file);
    let config_file = File::open("blacklist.json").unwrap();
    let _reader  = BufReader::new(config_file);
    let config: Config = serde_json::from_reader(reader).unwrap();
    Some(config)
}