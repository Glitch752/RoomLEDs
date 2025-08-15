#!/bin/bash

set -e

cd "$(dirname "$0")"

# Ensure docker is installed
if ! command -v docker &> /dev/null; then
  echo "Docker is not installed. Please install Docker to proceed."
  exit 1
fi

DEFAULT_TARGET_ARCH="aarch64-unknown-linux-gnu"
TARGET_ARCH="${TARGET_ARCH:-$DEFAULT_TARGET_ARCH}"
IMAGE_NAME="lights-controller"
IMAGE_TAG="${IMAGE_TAG:-latest}"

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
  local full_image_name="$IMAGE_NAME:$IMAGE_TAG"
  local cross_compile="false"
  
  # Determine if we need cross-compilation
  if [ "$TARGET_ARCH" = "aarch64-unknown-linux-gnu" ]; then
    cross_compile="true"
  fi
  
  echo "Building image $full_image_name for architecture $TARGET_ARCH"
  docker build \
    --build-arg TARGET_ARCH="$TARGET_ARCH" \
    --build-arg CROSS_COMPILE="$cross_compile" \
    -t "$full_image_name" \
    -f Dockerfile .
  
  echo "Image built successfully"
}

deploy_to_server() {
    if [ -z "$SERVER_IP" ] || [ -z "$SERVER_USER" ] || [ -z "$SERVER_IDENTITY_FILE" ]; then
        echo "SERVER_IP, SERVER_USER, and SERVER_IDENTITY_FILE must be set for deployment"
        exit 1
    fi
    
    local full_image_name="$IMAGE_NAME:$IMAGE_TAG"
    
    echo "Deploying to $SERVER_USER@$SERVER_IP"
  
    # Copy docker-compose.yml to server
    scp -i "$SERVER_IDENTITY_FILE" docker-compose.yml "$SERVER_USER@$SERVER_IP:~/lights-controller-compose.yml"
    
    # Update docker-compose.yml on server to use the built image
    ssh -i "$SERVER_IDENTITY_FILE" "$SERVER_USER@$SERVER_IP" "
        sed -i 's|build:.*|image: $full_image_name|' ~/lights-controller-compose.yml &&
        sed -i '/context:/d' ~/lights-controller-compose.yml &&
        sed -i '/dockerfile:/d' ~/lights-controller-compose.yml
    "
    
    docker save "$full_image_name" | gzip > /tmp/lights-controller.tar.gz

    echo "Transferring image to server"
    scp -i "$SERVER_IDENTITY_FILE" /tmp/lights-controller.tar.gz "$SERVER_USER@$SERVER_IP:~/lights-controller.tar.gz"

    echo "Loading image on server and starting container"
    ssh -i "$SERVER_IDENTITY_FILE" "$SERVER_USER@$SERVER_IP" "
        docker load < ~/lights-controller.tar.gz &&
        rm ~/lights-controller.tar.gz &&
        docker compose -f ~/lights-controller-compose.yml up -d
    "

    rm /tmp/lights-controller.tar.gz
}

main() {
  echo "Docker deployment for controller"
  echo "Target architecture: $TARGET_ARCH"
  echo "Image: $IMAGE_NAME:$IMAGE_TAG"
  
  build_image
  
  if [ "$BUILD_ONLY" = true ]; then
    echo "Build completed. Exiting as requested."
    exit 0
  fi
  
  if [ "$DEPLOY_TO_SERVER" = true ]; then
    deploy_to_server
  fi
  
  echo "Completed successfully!"
}

main "$@"
