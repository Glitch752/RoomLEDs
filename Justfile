install:
    cd controller/main/client && pnpm install

deploy:
    bash ./docker-deploy.sh --deploy

client:
    cd controller/main/client && pnpm dev

server:
    cd controller/main && cargo run --features localtest