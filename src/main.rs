use inputbot::{KeySequence, KeybdKey::*, MouseButton::*};
use std::{thread::sleep, time::Duration};

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
    
    // Bind the number 1 key your keyboard to a function that types 
    // "Hello, world!" when pressed.
    Numrow1Key.bind(|| {
        // call the function to get the name of the exe
        let exe_name = get_foreground_exe_name().unwrap();
        println!("exe_name: {}", exe_name);
    });

    // Call this to start listening for bound inputs.
    inputbot::handle_input_events();
}   

