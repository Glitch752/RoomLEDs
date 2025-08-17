#!/bin/bash

set -euo pipefail

cd "$(dirname "$0")"

# Ensure docker and docker-compose are installed
if ! command -v docker >/dev/null 2>&1; then
    echo "Docker is not installed. Please install Docker."
    exit 1
fi
if ! command -v docker-compose >/dev/null 2>&1; then
    echo "Docker Compose is not installed. Please install Docker Compose."
    exit 1
fi

DEFAULT_TARGET_ARCH="aarch64-unknown-linux-gnu"
TARGET_ARCH="${TARGET_ARCH:-$DEFAULT_TARGET_ARCH}"

APP_NAME="lights-controller"
IMAGE_NAME="${APP_NAME}:latest"

DEPLOY_TO_SERVER=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --deploy)
            DEPLOY_TO_SERVER=true
            shift
        ;;
        --target-arch)
            if [ "$2" = "x86" ]; then
                TARGET_ARCH="x86_64-unknown-linux-gnu"
            else
                TARGET_ARCH="$2"
            fi
            shift 2
        ;;
        --image-tag)
            IMAGE_TAG="$2"
            shift 2
        ;;
        *)
            echo "Unknown option $1"
            exit 1
        ;;
    esac
done

build_image() {
    local cross_compile="false"
    
    # Determine if we need cross-compilation
    if [ "$TARGET_ARCH" = "aarch64-unknown-linux-gnu" ]; then
        cross_compile="true"
    fi
    
    echo "Building image $IMAGE_NAME for architecture $TARGET_ARCH"
    docker build \
        --build-arg TARGET_ARCH="$TARGET_ARCH" \
        --build-arg CROSS_COMPILE="$cross_compile" \
        -t "$IMAGE_NAME" \
        -f Dockerfile .
    
    echo "Image built successfully"
}

deploy_to_server() {
    if [ -z "${SERVER_IP:-}" ] || [ -z "${SERVER_USER:-}" ] || [ -z "${SERVER_IDENTITY_FILE:-}" ]; then
        echo "SERVER_IP, SERVER_USER, and SERVER_IDENTITY_FILE must be set for deployment."
        exit 1
    fi

    remote_ssh="${SERVER_USER}@${SERVER_IP}"
    remote_context="remote-${APP_NAME}"

    # Make sure the identity file is in ~/.ssh/config
    if [ ! -f "$SERVER_IDENTITY_FILE" ]; then
        echo "Identity file $SERVER_IDENTITY_FILE does not exist."
        exit 1
    fi
    if ! grep -q "IdentityFile $SERVER_IDENTITY_FILE" ~/.ssh/config; then
        echo "Adding IdentityFile $SERVER_IDENTITY_FILE to ~/.ssh/config"
        {
            echo "Host $SERVER_IP"
            echo "    User $SERVER_USER"
            echo "    IdentityFile $SERVER_IDENTITY_FILE"
        } >> ~/.ssh/config
    fi

    # Set up passwordless sudo if required
    if ! ssh "$remote_ssh" "sudo -n true" 2>/dev/null; then
        # Confirm with the user they're fine compromising the security of this machine
        echo "This script will set up passwordless sudo on the remote machine. This is probably not secure, but it shouldn't matter too much for a lighting controller. Are you sure you want to continue? (y/N)"
        read -r confirmation
        if [[ ! "$confirmation" =~ ^[Yy]$ ]]; then
            echo "Aborting deployment."
            exit 1
        fi

        echo "Setting up passwordless sudo on remote server"
        ssh -t "$remote_ssh" "echo '$SERVER_USER ALL=(ALL) NOPASSWD: ALL' | sudo tee /etc/sudoers.d/$SERVER_USER"
    fi

    # Ensure the docker context exists
    if ! docker context inspect "$remote_context" >/dev/null 2>&1; then
        echo "Creating docker context $remote_context"
        docker context create "$remote_context" --docker "host=ssh://$remote_ssh"
    fi
    
    echo "Deploying to $remote_ssh"
  
    echo "Checking if Docker is installed remotely"
    ssh "$remote_ssh" <<'EOF'
    set -e
    if ! command -v docker >/dev/null 2>&1; then
        echo "Installing Docker"
        curl -fsSL https://get.docker.com | sudo sh
    fi
    if ! docker compose version >/dev/null 2>&1; then
        echo "Installing Docker Compose plugin"
        sudo apt-get update && sudo apt-get install -y docker-compose-plugin
    fi
EOF

    docker-compose --context "$remote_context" up -d

    echo "Deployment completed successfully on $remote_ssh"
}

main() {
    echo "Docker deployment for controller"
    echo "Target architecture: $TARGET_ARCH"
    echo "Image: $IMAGE_NAME"
    
    build_image
    
    if [ "$DEPLOY_TO_SERVER" = true ]; then
        deploy_to_server
    fi
    
    echo "Completed successfully!"
}

main "$@"
