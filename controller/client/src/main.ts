import './style.css'

const websocket = new WebSocket("ws://localhost:3000/websocket");

websocket.onopen = () => {
    console.log("Connection opened");
    websocket.send("Hello from the client!");
};
websocket.onclose = () => {
    console.log("Connection closed");
};
websocket.onmessage = (e) => {
    console.log(`Received message: ${e.data}`);
};