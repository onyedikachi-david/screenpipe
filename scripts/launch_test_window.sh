#!/bin/bash

# Check if running on Ubuntu
if [[ "$(uname)" == "Linux" ]]; then
  # Launch xterm with a visible window and some content
  xterm -geometry 80x24+100+100 -e "echo 'Test Window'; while true; do date; sleep 1; done" &
fi