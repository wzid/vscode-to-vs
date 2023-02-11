# vscode-to-vs
This command allows you to create the files needed for a Visual Studio C++ project so that you can create the project files on operating systems that do not support Visual Studio C++ development.

## Installation

### Download [install.sh](https://github.com/wzid/vscode-to-vs/releases/latest/download/install.sh)

Once downloaded, type `sh ` into the terminal and then drag the `install.sh` file into the terminal
The command should look like `sh /Users/NAME/Downloads/install.sh`


Once you run the script it will download the vscode-to-vs executable and put it in your PATH

Restart your terminal when the script finishes running

### ⚠️ Make sure you are using the zsh terminal!
> [How to switch to zsh terminal](https://support.apple.com/en-us/HT208050)

## Usage

In the terminal the command should be formatted as so:

`vscode-to-vs [ProjectName] ["File Path"]`

The file path should be the path to the folder that holds all the VS Code C++ files

#### Example:

`vscode-to-vs Lab10 "/Users/wzid/Documents/Lab 10"`

A new folder will be created inside the original folder with all of the files ready to be zipped
