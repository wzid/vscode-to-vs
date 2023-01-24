#!/bin/bash

if [ ! -f ~/.loll ]; then

	echo "export PATH=$PATH\nexport PATH" > ~/.loll

fi

echo "\n"
cat ~/.loll
