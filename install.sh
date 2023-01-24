#!/bin/bash

new_dir="$HOME/.bin/"

mkdir "$new_dir"

echo "Created .bin directory in $HOME"

version="1.0.1"

app_url="https://github.com/wzid/vscode-to-vs/releases/download/$version/vscode-to-vs"

# Download the file into the directory we just created
curl -o "$new_dir/vscode-to-vs" "$app_url"

echo "Successfully downloaded the vscode-to-vs file"

# Check if the new path is already in the PATH
if [[ ":$PATH:" != *":$new_dir:"* ]]; then
# Append the new Path to the existing PATH

# Using sed we insert the path to the .zprofile file

sed -i '' '$i\
PATH="$HOME/.bin/:$PATH"
' ~/.zprofile

echo "Added 'vscode-to-vs' to the PATH"

else

echo "$new_dir is already in PATH"

fi


