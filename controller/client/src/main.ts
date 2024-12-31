import './style.css'

const websocket = new WebSocket(`ws://${window.location.host}/websocket`);
const status = document.getElementById("status") as HTMLDivElement;

websocket.onopen = () => {
    console.log("Connection opened");
    websocket.send("Hello from the client!");
};
websocket.onclose = () => {
    console.log("Connection closed");
};
websocket.onmessage = (e) => {
    const data = JSON.parse(e.data);
    status.innerHTML = `
    Frames: ${data.frames}<br>
    <br>
    <b>Frame times over the last ${data.average_window} frames:</b><br>
    Average frame time: ${Math.round(data.average_frame_time * 1000 * 10) / 10}ms (${Math.round(1 / data.average_frame_time)}fps)<br>
    Max frame time: ${Math.round(data.max_frame_time * 1000 * 10) / 10}ms (${Math.round(1 / data.max_frame_time)}fps)<br>
    Min frame time: ${Math.round(data.min_frame_time * 1000 * 10) / 10}ms (${Math.round(1 / data.min_frame_time)}fps)<br>
`;
};