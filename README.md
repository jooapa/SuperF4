# SuperF4

SuperF4 is a small utility that allows you to quickly and easily close an unresponsive window on your Windows machine. 

SuperF4 also allows you to blacklist certain windows so that they are not closed when the key combination is pressed. This is useful if you have a window that you don't want to accidentally close, such as a game or a video player.

## Usage

SuperF4 is set up to listen for the **Right Ctrl + F11 combination**. When this combination is pressed it will forcefully terminate the active window. That is not listed in the `blacklist.json` file.

This is done by adding the (game).exe to the `blacklist.json` file in the same directory as the exe file.

Pressing the **Right Ctrl + F10 combination** will terminate the active window, even if it is listed in the `blacklist.json` file.

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

`./target/release/superf4.exe` 
*make sure, that the `blacklist.json` file is in the same directory as the exe file.*


