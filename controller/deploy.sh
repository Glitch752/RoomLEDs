# Deploy to the server

# Ensure bash is being used
if [ -z "$BASH_VERSION" ]; then
  echo "Please run this script using bash"
  exit 1
fi

# Make sure the current working directory is correct
if [ ! -f "./install.sh" ]; then
  echo "Please run this script from the controller directory"
  exit 1
fi

# Ensure SERVER_IP, SERVER_USER, and $SERVER_IDENTITY_FILE are set
if [ -z "$SERVER_IP" ] || [ -z "$SERVER_USER" ] || [ -z "$SERVER_IDENTITY_FILE" ]; then
  echo "SERVER_IP and SERVER_USER must be set"
  exit 1
fi

# If PASSWORD isn't set, prompt the user for it
if [ -z "$PASSWORD" ]; then
  read -s -p "Enter the password for $SERVER_USER: " PASSWORD
fi

# Deploy the controller
CONTROLLER_DIR="/home/$SERVER_USER/controller"
echo "Deploying the controller to $SERVER_USER@$SERVER_IP"

# If the folder already exists, remove it, and ensure rsync is installed
ssh -i "$SERVER_IDENTITY_FILE" "$SERVER_USER@$SERVER_IP" "rm -rf $CONTROLLER_DIR; if ! command -v rsync &> /dev/null; then echo \"$PASSWORD\" | sudo -S apt-get install rsync -y; fi"

echo "Removed the existing controller folder and ensured rsync is installed"

# Copy the controller folder
rsync -e "ssh -i '$SERVER_IDENTITY_FILE'" -ratlz --exclude='/.git' --filter="dir-merge,- .gitignore" . "$SERVER_USER@$SERVER_IP:$CONTROLLER_DIR"

echo "Copied the controller folder"

# Run the install script on the server
ssh -i "$SERVER_IDENTITY_FILE" "$SERVER_USER@$SERVER_IP" "cd $CONTROLLER_DIR && PASSWORD=\"$PASSWORD\" bash install.sh"

echo "Controller deployed to $SERVER_USER@$SERVER_IP"