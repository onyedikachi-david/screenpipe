#!/bin/bash

# Check if running on Ubuntu
if [[ "$(uname)" == "Linux" ]]; then
  # Function to simulate mouse movement
  simulate_mouse() {
    for i in {1..20}; do
      xdotool mousemove $((RANDOM % 1024)) $((RANDOM % 768))
      sleep 0.2
    done
  }

  # Function to simulate typing
  simulate_typing() {
    xdotool key Tab  # Focus on the window
    xdotool type "Screenpipe Test Content"
    xdotool key Return
    xdotool type "This is a test of screenpipe CLI"
    xdotool key Return
  }

  # Function to capture screenshot
  capture_screenshot() {
    xwd -root -silent | convert xwd:- png:/tmp/screenshot.png
  }

  # Main simulation loop
  while true; do
    simulate_mouse
    simulate_typing
    capture_screenshot
    sleep 1
  done
fi