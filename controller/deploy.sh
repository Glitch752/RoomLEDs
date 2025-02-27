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
  echo "SERVER_IP, SERVER_USER, and SERVER_IDENTITY_FILE must be set"
  exit 1
fi

# If PASSWORD isn't set, prompt the user for it
if [ -z "$PASSWORD" ]; then
  read -s -p "Enter the password for $SERVER_USER: " PASSWORD
fi

# If the first parameter is "quick", run a quick install instead of a full one
if [ "$1" == "quick" ]; then
  echo "Running a quick install"

  quick_install="true"
  shift
fi

# Build the client
echo "Building the client"
cd client || exit 1
pnpm run build
cd .. || exit 1

# Deploy the controller
CONTROLLER_DIR="/home/$SERVER_USER/controller"
echo "Deploying the controller to $SERVER_USER@$SERVER_IP"

# We sadly can't cross-compile, so we need to build the controller on the server
# If doing a quick deploy, we can assume dependencies are already installed

if [ -z "$quick_install" ]; then
  # Ensure rsync is installed
  ssh -i "$SERVER_IDENTITY_FILE" "$SERVER_USER@$SERVER_IP" "if ! command -v rsync &> /dev/null; then echo \"$PASSWORD\" | sudo -S apt-get install rsync -y; fi"

  echo "Ensured rsync is installed"
fi

# Copy the controller folder, including static even though it's ignored in .gitignore
rsync -e "ssh -i '$SERVER_IDENTITY_FILE'" \
  --exclude='/.git' --exclude='/client' --include='/static' --filter="dir-merge,- .gitignore" \
  --update -ratlz \
  . "$SERVER_USER@$SERVER_IP:$CONTROLLER_DIR"

# We also need to copy the shared folder
rsync -e "ssh -i '$SERVER_IDENTITY_FILE'" \
  --update -ratlz \
  ../shared/ "$SERVER_USER@$SERVER_IP:/home/$SERVER_USER/shared"

# ...and the reflection folder
rsync -e "ssh -i '$SERVER_IDENTITY_FILE'" \
  --update -ratlz \
  ../reflection/ "$SERVER_USER@$SERVER_IP:/home/$SERVER_USER/reflection"

echo "Copied the controller folder"

PARAMS=""
if [ -n "$quick_install" ]; then
  PARAMS="quick"
fi

# Run the install script on the server
ssh -i "$SERVER_IDENTITY_FILE" "$SERVER_USER@$SERVER_IP" "cd $CONTROLLER_DIR && PASSWORD=\"$PASSWORD\" bash install.sh $PARAMS"

echo "Controller deployed to $SERVER_USER@$SERVER_IP"