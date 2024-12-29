#!/bin/bash

# Ensure bash is being used
if [ -z "$BASH_VERSION" ]; then
  echo "Please run this script using bash"
  exit 1
fi

# Make sure the current working directory is correct
if [ ! -f "controller/install.sh" ]; then
  echo "Please run this script from the outer directory"
  exit 1
fi

# Make sure the script isn't run as root
if [ "$EUID" -eq 0 ]; then
  echo "Please run this script as a non-root user"
  exit 1
fi

# If the `curl` command doesn't exist, install it
if ! command -v curl &> /dev/null; then
  echo "Installing curl..."
  sudo apt-get install curl -y
fi

# If the `rye` command doesn't exist, install Rye
if ! command -v rye &> /dev/null; then
  echo "Installing Rye..."
  curl -sSf https://rye.astral.sh/get | bash
  source "$HOME/.rye/env"
fi

# Set up a systemd service to run the controller if it doesn't already exist
SYSTEMD_SERVICE_PATH="/etc/systemd/system/rye-controller.service"

sudo cp controller/rye-controller.service $SYSTEMD_SERVICE_PATH
if [ ! -f "$SYSTEMD_SERVICE_PATH" ]; then
  echo "Setting up systemd service..."
  sudo systemctl enable rye-controller
  sudo systemctl start rye-controller
else
  echo "Systemd service already exists; reloading..."
  sudo systemctl daemon-reload
  sudo systemctl restart rye-controller
fi