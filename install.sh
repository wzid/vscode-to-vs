#!/bin/bash

new_dir="$HOME/.bin"

mkdir -p "$new_dir"

echo "Created .bin directory in ~\n"

app_url="https://github.com/wzid/vscode-to-vs/releases/latest/download/vscode-to-vs"

# Download the file into the directory we just created
curl -Lo "$new_dir/vscode-to-vs" "$app_url"

chmod 777 "$new_dir/vscode-to-vs"

echo "Successfully downloaded the vscode-to-vs file\n"

# Check if the .zprofile exists
if [ ! -f ~/.zprofile ]; then
	# Add the current PATH to the terminal
	echo "export PATH=$PATH\nexport PATH" > ~/.zprofile

fi



# Check if the new path is already in the PATH
if [[ ":$PATH:" != *":$new_dir/:"* ]]; then
# Append the new Path to the existing PATH

# Using sed we insert the path to the .zprofile file

sed -i '' '$i\
PATH="$HOME/.bin/:$PATH"
' ~/.zprofile

echo "Added 'vscode-to-vs' to the PATH\n"

else

echo "$new_dir is already in PATH\n"

fi
