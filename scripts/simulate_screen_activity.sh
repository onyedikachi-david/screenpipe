#!/bin/bash

# Check if running on Ubuntu
if [[ "$(uname)" == "Linux" ]]; then
  # Create a simple window
  xterm -e "echo 'Screenpipe Test Window'; sleep 60" &

  # Simulate mouse movement
  for i in {1..10}; do
    xdotool mousemove $((RANDOM % 1920)) $((RANDOM % 1080))
    sleep 1
  done

  # Simulate typing
  xdotool type "This is a test of screenpipe CLI"
  xdotool key Return
fi