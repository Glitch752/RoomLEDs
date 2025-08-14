#!/bin/bash

set -e

# Make sure the current working directory is correct
if [ ! -f "./docker-deploy.sh" ]; then
  echo "Please run this script from the controller directory"
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
      TARGET_ARCH="$2"
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

update_dockerfile_target() {
  local arch=$1
  local dockerfile="./Dockerfile"
  
  case $arch in
    "aarch64-unknown-linux-gnu")
      sed -i 's/gcc-[a-z0-9-]*/gcc-aarch64-linux-gnu/g' "$dockerfile"
      sed -i 's/CARGO_TARGET_[A-Z0-9_]*_LINKER/CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER/g' "$dockerfile"
      sed -i 's/CC_[a-z0-9-]*/CC_aarch64_unknown_linux_gnu/g' "$dockerfile"
      sed -i 's/CXX_[a-z0-9-]*/CXX_aarch64_unknown_linux_gnu/g' "$dockerfile"
      sed -i 's/aarch64-linux-gnu-gcc/aarch64-linux-gnu-gcc/g' "$dockerfile"
      sed -i 's/--target [a-z0-9-]*/--target aarch64-unknown-linux-gnu/g' "$dockerfile"
      ;;
    "x86_64-unknown-linux-gnu")
      # For x86_64, we don't need cross-compilation tools
      sed -i 's/gcc-aarch64-linux-gnu //g' "$dockerfile"
      sed -i '/ENV CARGO_TARGET_.*_LINKER/d' "$dockerfile"
      sed -i '/ENV CC_.*=/d' "$dockerfile"
      sed -i '/ENV CXX_.*=/d' "$dockerfile"
      sed -i 's/--target aarch64-unknown-linux-gnu//g' "$dockerfile"
      ;;
    *)
      echo "Unsupported target architecture $arch"
      exit 1
      ;;
  esac
}

build_image() {
  update_dockerfile_target "$TARGET_ARCH"
  
  # Build the image
  local full_image_name="$IMAGE_NAME:$IMAGE_TAG"
  
  echo "Building image $full_image_name"
  docker build -t "$full_image_name" -f Dockerfile ..
  
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
    
    echo "Saving image to transfer to server..."
    docker save "$full_image_name" | gzip > /tmp/lights-controller.tar.gz

    echo "Transferring image to server..."
    scp -i "$SERVER_IDENTITY_FILE" /tmp/lights-controller.tar.gz "$SERVER_USER@$SERVER_IP:~/lights-controller.tar.gz"

    echo "Loading image on server and starting container..."
    ssh -i "$SERVER_IDENTITY_FILE" "$SERVER_USER@$SERVER_IP" "
        docker load < ~/lights-controller.tar.gz &&
        rm ~/lights-controller.tar.gz &&
        docker compose -f ~/lights-controller-compose.yml up -d
    "

    # Clean up local file
    rm /tmp/lights-controller.tar.gz
  fi
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
