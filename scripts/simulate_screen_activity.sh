#!/bin/bash

# Check if running on Ubuntu
if [[ "$(uname)" == "Linux" ]]; then
  # Create a simple text file with some content
  echo "Screenpipe Test Content" > test_content.txt
  echo "This is a test of screenpipe CLI" >> test_content.txt
  
  # Display the content using cat (this will be visible in the virtual framebuffer)
  cat test_content.txt

  # Keep the script running for a while to allow screen capture
  sleep 60
fi