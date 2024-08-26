#!/bin/bash

# Check if running on Ubuntu
if [[ "$(uname)" == "Linux" ]]; then
  # Launch xterm with a visible window
  xterm -geometry 80x24+100+100 -e "echo 'Test Window'; sleep 300" &
fi