# SuperF4

SuperF4 is a small utility that allows you to quickly and easily close an unresponsive window on your Windows machine. With SuperF4, you can close an unresponsive window with just a couple of keystrokes.
**make sure the blacklist.json file is in the same directory as the exe file**

## Features

SuperF4 is a lightweight utility that runs in the background and listens for a specific key combination. By default, SuperF4 is set up to listen for the Scroll Lock + F12 combination. When this combination is pressed, SuperF4 will use the `taskkill` command to forcefully terminate the active window.

## Usage

### Downloading the Setup

The easiest way to use SuperF4 is to download the latest release from the [releases page](https://github.com/jooapa/SuperF4/releases/tag/release). Once you've downloaded the setup, simply run it and follow the instructions to install SuperF4.

### Building from Source

If you prefer to build SuperF4 from source, you'll need to have [Rust](https://www.rust-lang.org/tools/install) installed on your system. Once you have Rust installed, follow these steps:

1.  Clone the repository to your local machine:

`git clone https://github.com/jooapa/SuperF4.git` 

2.  Change to the SuperF4 directory:

`cd SuperF4` 

3.  Build SuperF4 using Cargo:

`cargo build --release` 

4.  Run SuperF4:

arduinoCopy code

`./target/release/superf4.exe` 

By default, SuperF4 will listen for the Scroll Lock + F12 combination. If you want to use a different key combination, you can edit the `main.rs` file in the root directory of the repository. 


