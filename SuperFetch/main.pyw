from pynput.keyboard import Listener
from pynput.keyboard import Key

import psutil
import platform
import subprocess

import pygetwindow as gw
import psutil
import ctypes
import os

import json
import pyautogui
import time

#f = open('data.json') # open file but if it doesn't exist, create it and write {"blacklist": ["explorer.exe"],"speed": [14]}
if not os.path.exists('options.json'):
    with open('options.json', 'w') as f:
        json.dump({"blacklist": ["explorer.exe"],"speed": [14]}, f)

def get_active_window_exe():
    try:
        active_window = gw.getActiveWindow()
        hwnd = active_window._hWnd
        pid = ctypes.c_ulong()
        ctypes.windll.user32.GetWindowThreadProcessId(hwnd, ctypes.byref(pid))
        window_process = psutil.Process(pid.value)
        exe_path = window_process.exe()
        return os.path.basename(exe_path)
    except (gw.PyGetWindowException, psutil.NoSuchProcess):
        return None
    
def close_program(program_name):
    try:
        if platform.system() == "Windows":
            subprocess.run(['taskkill', '/f', '/im', program_name], check=True)
        elif platform.system() == "Darwin" or platform.system() == "Linux":
            for proc in psutil.process_iter(['pid', 'name']):
                if program_name in proc.info['name']:
                    proc.terminate()
                    break
        else:
            print("Unsupported platform.")
            return

        print(f"{program_name} has been closed.")
    except (subprocess.CalledProcessError, psutil.NoSuchProcess):
        print(f"Failed to close {program_name}. Maybe it's not running.")
        
def on_press(key):
    global ctrl_r_pressed  # Declare the variable as global within the function
    global autoclicker_active
    try:
        if key == Key.ctrl_r:
            ctrl_r_pressed = True
            print("Ctrl+R is being held down. Press F11 to close the program.")
        elif ctrl_r_pressed and key == Key.f11:
            #if in blacklist, don't close
            with open('options.json') as f:
                options = json.load(f)
            blacklist = options["blacklist"]
            if get_active_window_exe() in blacklist:
                print("Blacklisted program. Not closing.")
                return
            #close program
            close_program(get_active_window_exe())

            ctrl_r_pressed = False

        #autoclicker activated if f8 is pressed
        elif key == Key.f8:
            with open('options.json') as f:
                options = json.load(f)
            speed = options["speed"]
            print("Autoclicker activated. Press F8 again to deactivate.")
            while True:
                print("Clicking...")
                if not key == Key.f8:
                    print("Autoclicker deactivated.")
                    break
            

    except AttributeError:
        pass

def on_release(key):
    if key == Key.ctrl_r:
        ctrl_r_pressed = False
    if key == Key.esc:
        return False
    
if __name__ == "__main__":
    #open note
    subprocess.Popen(["notepad.exe"])
    global ctrl_r_pressed
    ctrl_r_pressed = False
    with Listener(on_press=on_press, on_release=on_release) as listener:
        listener.join()