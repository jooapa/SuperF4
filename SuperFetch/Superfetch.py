from pynput.keyboard import Listener
from pynput.keyboard import Key

import psutil
import platform
import subprocess

import pygetwindow as gw
import psutil
import ctypes
import os
import threading

import mouse

import json
import time

#f = open('data.json') # open file but if it doesn't exist, create it and write {"blacklist": ["explorer.exe"],"speed": [14]}
if not os.path.exists('preferences.json'):
    with open('preferences.json', 'w') as f:
        json.dump({"blacklist": ["explorer.exe"],"speed": [2]}, f)

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
        
def autoclick():
    global autoclicker_active
    global speed

    while autoclicker_active:
        mouse.click(button='left')
        time.sleep(speed / 1000)

def on_press(key):
    global ctrl_r_pressed
    global autoclicker_active
    global speed

    try:
        if key == Key.ctrl_r:
            ctrl_r_pressed = True
            print("Ctrl+R is being held down. Press F11 to close the program.")

        elif ctrl_r_pressed and key == Key.f11:
            with open('preferences.json') as f:
                options = json.load(f)
            blacklist = options["blacklist"]
            if get_active_window_exe() in blacklist:
                print("Blacklisted program. Not closing.")
                return
            close_program(get_active_window_exe())
            ctrl_r_pressed = False

        elif ctrl_r_pressed and key == Key.f10:
            with open('preferences') as f:
                options = json.load(f)
            blacklist = options["blacklist"]
            close_program(get_active_window_exe())
            ctrl_r_pressed = False

        elif ctrl_r_pressed and key == Key.f8:
            with open('preferences.json') as f:
                options = json.load(f)
            speed = options["speed"][0]
            print("Autoclicker activated. Press F8 again to deactivate.")
            autoclicker_active = not autoclicker_active
            if autoclicker_active:
                autoclick_thread = threading.Thread(target=autoclick)
                autoclick_thread.start()

    except AttributeError:
        pass

def on_release(key):
    global ctrl_r_pressed
    global autoclicker_active

    if key == Key.ctrl_r:
        ctrl_r_pressed = False

if __name__ == "__main__":
    global ctrl_r_pressed
    ctrl_r_pressed = False
    global autoclicker_active
    autoclicker_active = False
    with Listener(on_press=on_press, on_release=on_release) as listener:
        listener.join()

#make exe
#pyinstaller --onefile --noconsole --icon=assets/icon.ico .\Superfetch.py