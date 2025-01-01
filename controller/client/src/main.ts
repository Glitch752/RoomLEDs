import './style.css'

const websocket = new WebSocket(`ws://${window.location.host}/websocket`);
websocket.binaryType = "arraybuffer";

const status = document.getElementById("status") as HTMLDivElement;

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;

websocket.onopen = () => {
    console.log("Connection opened");
    websocket.send("Hello from the client!");
};
websocket.onclose = () => {
    console.log("Connection closed");
};
websocket.onmessage = (e) => {
    // If the message is a string, it's a JSON update. If binary, it's an update on the state of the lights.
    if(typeof e.data === "string") {
        const data = JSON.parse(e.data);
        status.innerHTML = `
Frames: ${data.frames}<br>
<br>
<b>Frame times over the last ${data.average_window} frames:</b><br>
Average frame time: ${Math.round(data.average_frame_time * 1000 * 10) / 10}ms (${Math.round(1 / data.average_frame_time)}fps)<br>
Max frame time: ${Math.round(data.max_frame_time * 1000 * 10) / 10}ms (${Math.round(1 / data.max_frame_time)}fps)<br>
Min frame time: ${Math.round(data.min_frame_time * 1000 * 10) / 10}ms (${Math.round(1 / data.min_frame_time)}fps)<br>`;
    } else if(e.data instanceof ArrayBuffer) {
        const data = new Uint8Array(e.data);

        ctx.clearRect(0, 0, canvas.width, canvas.height);
        for(let i = 0; i < data.length; i += 3) {
            const size = 2;
            const x = (i / 3) % (canvas.width / size);
            const y = Math.floor((i / 3) / (canvas.width / size));

            const r = data[i + 0];
            const g = data[i + 1];
            const b = data[i + 2];

            ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;
            ctx.fillRect(x * size, y * size, size, size);
        }
    }
};