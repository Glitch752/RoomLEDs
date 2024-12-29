#!/bin/bash

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

# If $PASSWORD is set, uses it as the sudo password.
# The deploy script sets this variable to the user's password.
sudo_pass() {
  if [ -n "$PASSWORD" ]; then
    echo "$PASSWORD" | sudo -S $@
  else
    sudo $@
  fi
}

# If the `curl` command doesn't exist, install it
if ! command -v curl &> /dev/null; then
  echo "Installing curl..."
  sudo_pass apt-get install curl -y
fi

# If "$HOME/.rye/env" exists, source it (since otherwise it doesn't source automatically when running ssh commands)
if [ -f "$HOME/.rye/env" ]; then
  source "$HOME/.rye/env"
fi

# If the `rye` command doesn't exist, install Rye
if ! command -v rye &> /dev/null; then
  echo "No Rye installation found; installing Rye..."
  curl -sSf https://rye.astral.sh/get | RYE_INSTALL_OPTION="--yes" bash
  source "$HOME/.rye/env"
fi

# Set up a systemd service to run the controller if it doesn't already exist
SYSTEMD_SERVICE_PATH="/etc/systemd/system/rye-controller.service"

# Make rye-controller.service
echo "Setting up systemd service..."
cat <<EOF > rye-controller.service
[Unit]
Description=Rye Controller
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

$(which rye) sync
$(which rye) run controller
EOF

sudo_pass cp ./rye-controller.service $SYSTEMD_SERVICE_PATH
if [ ! -f "$SYSTEMD_SERVICE_PATH" ]; then
  echo "Initial systemd service..."
  sudo_pass systemctl enable rye-controller
  sudo_pass systemctl start rye-controller
else
  echo "Systemd service already exists; reloading..."
  sudo_pass systemctl daemon-reload
  sudo_pass systemctl restart rye-controller
fi