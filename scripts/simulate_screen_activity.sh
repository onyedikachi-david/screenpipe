#!/bin/bash

# Check if running on Ubuntu
if [[ "$(uname)" == "Linux" ]]; then
  # Function to simulate mouse movement
  simulate_mouse() {
    for i in {1..10}; do
      xdotool mousemove $((RANDOM % 1024)) $((RANDOM % 768))
      sleep 0.5
    done
  }

  # Function to simulate typing
  simulate_typing() {
    xdotool type "Screenpipe Test Content"
    xdotool key Return
    xdotool type "This is a test of screenpipe CLI"
    xdotool key Return
  }

  # Main simulation loop
  for i in {1..6}; do
    simulate_mouse
    simulate_typing
    sleep 1
  done

  # Keep the script running for a while to allow screen capture
  sleep 30
fi