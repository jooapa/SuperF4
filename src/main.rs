use inputbot::{KeybdKey::*};

use std::cell::RefCell;
use std::sync::Mutex;

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


fn main() {
    let code_executed = Mutex::new(RefCell::new(false));

    ScrollLockKey.bind(move || {
        while ScrollLockKey.is_pressed() {
            if F12Key.is_pressed() && !*code_executed.lock().unwrap().borrow() {
                let exe_name = get_foreground_exe_name().unwrap();
                println!("exe_name: {}", exe_name);
                *code_executed.lock().unwrap().borrow_mut() = true;
            } else if F12Key.is_pressed() && *code_executed.lock().unwrap().borrow() {
                // F12 has already been pressed and code has already been executed,
                // so exit the loop early to avoid printing the message multiple times.
                break;
            }
        }
        *code_executed.lock().unwrap().borrow_mut() = false; // Reset the flag when Scroll Lock is released
    });
    
    inputbot::handle_input_events();
    
}