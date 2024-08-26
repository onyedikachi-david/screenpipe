#!/bin/bash

# Check if running on Ubuntu
if [[ "$(uname)" == "Linux" ]]; then
  # Create a simple window using xterm
  xterm -e "echo 'Screenpipe Test Window'; sleep 300" &
  
  # Wait for xterm to start
  sleep 2

  # Get the window ID of the xterm window
  WINDOW_ID=$(xdotool search --name "Screenpipe Test Window" | head -n 1)

  # Activate the window
  xdotool windowactivate $WINDOW_ID

  # Simulate mouse movement
  for i in {1..10}; do
    xdotool mousemove --window $WINDOW_ID $((RANDOM % 500)) $((RANDOM % 300))
    sleep 1
  done

  # Simulate typing
  xdotool type --window $WINDOW_ID "This is a test of screenpipe CLI"
  xdotool key --window $WINDOW_ID Return
fi