#!/bin/bash

# Check if running on Ubuntu
if [[ "$(uname)" == "Linux" ]]; then
  while true; do
    # Generate a simple tone and play it through PulseAudio
    paplay <(sox -n -r 44100 -c 1 -b 16 -t wav - synth 0.5 sine 440 synth 0.5 sine 494 synth 0.5 sine 523 synth 0.5 sine 587 synth 0.5 sine 659 synth 0.5 sine 698 synth 0.5 sine 784 synth 0.5 sine 880)
    sleep 1
  done
fi