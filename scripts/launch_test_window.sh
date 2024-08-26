#!/bin/bash

# Check if running on Ubuntu
if [[ "$(uname)" == "Linux" ]]; then
  # Create a simple GTK window
  python3 - << END
import gi
gi.require_version('Gtk', '3.0')
from gi.repository import Gtk, GLib

def main():
    window = Gtk.Window(title="Test Window")
    window.set_default_size(400, 300)
    window.connect("destroy", Gtk.main_quit)
    window.show_all()
    GLib.timeout_add_seconds(300, Gtk.main_quit)  # Exit after 5 minutes
    Gtk.main()

if __name__ == "__main__":
    main()
END
fi