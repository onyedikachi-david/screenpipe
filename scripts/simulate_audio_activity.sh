#!/bin/bash

# Check if running on Ubuntu
if [[ "$(uname)" == "Linux" ]]; then
  # Generate a silent audio file
  ffmpeg -f lavfi -i anullsrc=r=44100:cl=mono -t 10 -q:a 9 -acodec libmp3lame silence.mp3

  # Play the silent audio file (this will create audio activity without actual sound)
  ffplay -nodisp -autoexit silence.mp3
fi