#!/bin/bash

# Check if running on Ubuntu
if [[ "$(uname)" == "Linux" ]]; then
  # Play a test audio file through the virtual speaker
  ffplay -nodisp -autoexit -f lavfi -i sine=frequency=1000:duration=5 -af 'asetpts=PTS-STARTPTS' &
fi