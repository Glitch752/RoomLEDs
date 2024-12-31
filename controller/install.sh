#!/bin/bash

needs_reboot=false

# Ensure bash is being used
if [ -z "$BASH_VERSION" ]; then
  echo "Please run this script using bash"
  exit 1
fi

# Make sure the current working directory is correct
if [ ! -f "./install.sh" ]; then
  echo "Please run this script in the controller directory"
  exit 1
fi

# Make sure the script isn't run as root
if [ "$EUID" -eq 0 ]; then
  echo "Please run this script as a non-root user"
  exit 1
fi

# If the first parameter is "quick", run a quick install instead of a full one
if [ "$1" == "quick" ]; then
  echo "Running a quick install"

  quick_install="true"
  shift
fi

# If $PASSWORD is set, uses it as the sudo password.
# The deploy script sets this variable to the user's password.
sudo_pass() {
  if [ -n "$PASSWORD" ]; then
    echo "$PASSWORD" | sudo -S $@
  else
    sudo $@
  fi
}

# We use setcap to add the cap_sys_nice capability to the binary,
# which allows it to set the real-time priority of the thread.
add_executable_permissions() {
  echo "Adding executable permissions to $1..."
  sudo_pass setcap cap_sys_nice+ep $1
}

if [ -z "$quick_install" ]; then
  # If the `curl` command doesn't exist, install it
  if ! command -v curl &> /dev/null; then
    echo "Installing curl..."
    sudo_pass apt-get install curl -y
  fi

  # If pkg-config isn't installed, install it
  if ! command -v pkg-config &> /dev/null; then
    echo "Installing pkg-config..."
    sudo_pass apt-get install pkg-config -y
  fi

  # If libudev isn't installed, install it
  if ! dpkg -l | grep libudev-dev &> /dev/null; then
    echo "Installing libudev-dev..."
    sudo_pass apt-get install libudev-dev -y
  fi

  # If $HOME/.cargo/env exists, source it
  if [ -f "$HOME/.cargo/env" ]; then
    source $HOME/.cargo/env
  fi

  # If the `cargo` command doesn't exist, install Cargo and Rustc
  if ! command -v cargo &> /dev/null; then
    echo "Installing Cargo..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
  fi

  # Ensure the user has the dialout group
  if ! groups | grep dialout &> /dev/null; then
    echo "Adding the user to the dialout group..."
    sudo_pass usermod -aG dialout $USER
    needs_reboot=true
  fi

  # Build the project in release mode
  echo "Building the project in release mode..."
  cargo build --release

  # Add the required capabilities to the binary
  add_executable_permissions target/release/lights-controller

  # Set up a systemd service to run the controller if it doesn't already exist
  SYSTEMD_SERVICE_PATH="/etc/systemd/system/lights-controller.service"

  # Make lights-controller.service
  echo "Setting up systemd service..."
  cat <<EOF > lights-controller.service
[Unit]
Description=Room Lights Controller
After=network.target

[Service]
Type=simple
User=$USER
WorkingDirectory=$HOME/controller
ExecStart=bash $HOME/controller/run.sh
Restart=always

[Install]
WantedBy=multi-user.target
EOF

  # Write run.sh
  echo "Writing run.sh..."
  cat <<EOF > run.sh
#!/bin/bash

./target/release/lights-controller
EOF

  if [ ! -f "$SYSTEMD_SERVICE_PATH" ]; then
    echo "Initial systemd service..."

    # Move the systemd service to the correct location
    sudo_pass mv lights-controller.service $SYSTEMD_SERVICE_PATH

    sudo_pass systemctl enable lights-controller
    sudo_pass systemctl start lights-controller
  else
    echo "Systemd service already exists; reloading..."

    # Move the systemd service to the correct location
    sudo_pass mv lights-controller.service $SYSTEMD_SERVICE_PATH

    sudo_pass systemctl daemon-reload
    sudo_pass systemctl restart lights-controller
  fi
else
  # Build the project in debug mode
  echo "Building the project in debug mode..."

  # Source the Cargo environment
  source $HOME/.cargo/env

  cargo build

  # Add the required capabilities to the binary
  add_executable_permissions target/debug/lights-controller

  # Write run.sh
  echo "Writing run.sh..."
  cat <<EOF > run.sh
#!/bin/bash

./target/debug/lights-controller
EOF

  # Restart the systemd service
  sudo_pass systemctl restart lights-controller
fi

# If the user was added to the dialout group, a reboot is needed
if [ "$needs_reboot" = true ]; then
  echo "A reboot is needed for the changes to take effect"
  sudo_pass reboot
fi