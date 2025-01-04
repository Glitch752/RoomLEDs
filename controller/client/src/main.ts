import './style.css'

const websocket = new WebSocket(`ws://${window.location.host}/websocket`);
websocket.binaryType = "arraybuffer";

const status = document.getElementById("status") as HTMLDivElement;

const canvas = document.getElementById("canvas") as HTMLCanvasElement;
const ctx = canvas.getContext("2d")!;

type LightPositions = {
    positions: [number, number][],
    xMin: number,
    xMax: number,
    yMin: number,
    yMax: number
};
let lightPositions: LightPositions | null = null;

(async () => {
    await fetch("/api/light_positions").then(res => res.json()).then((data: [number, number][]) => {
        let xMin = Infinity, xMax = -Infinity, yMin = Infinity, yMax = -Infinity;
        for(const [x, y] of data) {
            xMin = Math.min(xMin, x);
            xMax = Math.max(xMax, x);
            yMin = Math.min(yMin, y);
            yMax = Math.max(yMax, y);
        }

        const margin = 0.1;
        xMin -= (xMax - xMin) * margin;
        xMax += (xMax - xMin) * margin;
        yMin -= (yMax - yMin) * margin;
        yMax += (yMax - yMin) * margin;

        lightPositions = {
            positions: data,
            xMin, xMax, yMin, yMax
        };
    });
})();

function getLightPosition(index: number): { x: number, y: number } {
    if(lightPositions === null) {
        return { x: 0, y: 0 };
    }

    const { positions, xMin, xMax, yMin, yMax } = lightPositions;
    const [x, y] = positions[index % positions.length];

    let scaleFactor = Math.min(canvas.width / (xMax - xMin), canvas.height / (yMax - yMin));
    if(scaleFactor === Infinity) {
        scaleFactor = 1;
    }

    return {
        x: (x - xMin) * scaleFactor,
        y: (y - yMin) * scaleFactor
    };
}

websocket.onopen = () => {
    console.log("Connection opened");
    websocket.send("Hello from the client!");
};
websocket.onclose = () => {
    console.log("Connection closed");
};
websocket.onmessage = (e: MessageEvent) => {
    // If the message is a string, it's a JSON update. If binary, it's an update on the state of the lights.
    if(typeof e.data === "string") {
        const data = JSON.parse(e.data);
        currentData = mergeIntoObject(currentData, data);
        updateStatus();
    } else if(e.data instanceof ArrayBuffer) {
        const data = new Uint8Array(e.data);

        ctx.clearRect(0, 0, canvas.width, canvas.height);
        for(let i = 0; i < data.length; i += 3) {
            const size = 2;

            const { x, y } = getLightPosition(i / 3);

            const r = data[i + 0];
            const g = data[i + 1];
            const b = data[i + 2];

            ctx.fillStyle = `rgb(${r}, ${g}, ${b})`;

            ctx.beginPath();
            ctx.arc(x, y, size, 0, Math.PI * 2);
            ctx.fill();
        }
    }
};

function mergeIntoObject<T>(obj: T, ...sources: Partial<T>[]): T {
    for(const source of sources) {
        for(const key in source) {
            if(source[key] !== undefined) {
                obj[key] = source[key];
            }
        }
    }
    return obj;
}

type LightingData = {
    frames: number;
    average_window: number;
    average_frame_time: number;
    max_frame_time: number;
    min_frame_time: number;
    debug_text: string;
    system: {
        global_cpu: number;
        free_memory: number;
        total_memory: number;
        used_swap: number;
    };
};
let currentData: LightingData = {
    frames: 0,
    average_window: 0,
    average_frame_time: 0,
    max_frame_time: 0,
    min_frame_time: 0,
    debug_text: "",
    system: {
        global_cpu: 0,
        free_memory: 0,
        total_memory: 0,
        used_swap: 0
    }
};

function updateStatus() {
    const data = currentData;

    status.innerHTML = `
Frames: ${data.frames}<br>
<br>
<b>Frame times over the last ${data.average_window} frames:</b><br>
Average frame time: ${Math.round(data.average_frame_time * 1000 * 10) / 10}ms (${Math.round(1 / data.average_frame_time)}fps)<br>
Max frame time: ${Math.round(data.max_frame_time * 1000 * 10) / 10}ms (${Math.round(1 / data.max_frame_time)}fps)<br>
Min frame time: ${Math.round(data.min_frame_time * 1000 * 10) / 10}ms (${Math.round(1 / data.min_frame_time)}fps)<br>
<br>
Debug text: ${data.debug_text}<br>
<br>
<b>System:</b><br>
Global CPU: ${Math.round(data.system.global_cpu * 10) / 10}%<br>
Free memory: ${Math.round(data.system.free_memory / 1024 / 1024)}MB / ${Math.round(data.system.total_memory / 1024 / 1024)}MB (${Math.round(data.system.free_memory / data.system.total_memory * 100)}%)<br>
Used swap: ${Math.round(data.system.used_swap / 1024 / 1024 * 10) / 10}MB
`;
}