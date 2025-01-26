import type { LightPosition } from "@shared-bindings/LightPosition";
import type { ServerToClientMessage } from "@shared-bindings/ServerToClientMessage";
import type { StatusUpdateMessage } from "@shared-bindings/StatusUpdateMessage";
import type { SystemStatusUpdateMessage } from "@shared-bindings/SystemStatusUpdateMessage";
import { writable } from "svelte/store";

const websocket = new WebSocket(`ws://${window.location.host}/websocket`);
websocket.binaryType = "arraybuffer";

export type LightPositionData = {
    positions: LightPosition[],
    xMin: number,
    xMax: number,
    yMin: number,
    yMax: number
};

export let lightPositions: LightPositionData | null = null;
export let lightData = new Uint8Array(0);
export let statusMessage = writable("");

let currentData: StatusUpdateMessage = {
    frames: 0,
    average_window: 0,
    average_frame_time: 0,
    max_frame_time: 0,
    min_frame_time: 0,
    debug_text: ""
};
let currentSystemData: SystemStatusUpdateMessage = {
    global_cpu: 0,
    available_memory: 0,
    total_memory: 0,
    used_swap: 0
};

websocket.onopen = () => {
    console.log("Connection opened");
};
websocket.onclose = () => {
    console.log("Connection closed");
};
websocket.onmessage = (e: MessageEvent) => {
    // If the message is a string, it's a JSON update. If binary, it's an update on the state of the lights.
    if(typeof e.data === "string") {
        const data: ServerToClientMessage = JSON.parse(e.data);
        switch(data.type) {
            case "Initialize":
                let minX = Infinity, maxX = -Infinity, minY = Infinity, maxY = -Infinity;
                for(const light of data.light_positions) {
                    minX = Math.min(minX, light.x);
                    maxX = Math.max(maxX, light.x);
                    minY = Math.min(minY, light.y);
                    maxY = Math.max(maxY, light.y);
                }
                lightPositions = {
                    positions: data.light_positions,
                    xMin: minX,
                    xMax: maxX,
                    yMin: minY,
                    yMax: maxY
                };
                break;
            case "StatusUpdate":
                currentData = data;
                updateStatus();
                break;
            case "SystemStatusUpdate":
                currentSystemData = data;
                updateStatus();
                break;
            default:
                const _exhaustiveCheck: never = data;
                console.error("Unhandled message type: ", data);
                break;
        }
    } else if(e.data instanceof ArrayBuffer) {
        lightData = new Uint8Array(e.data);
    }
};

// Send light data to the server; sent data will
// be rendered by websocket input effects.
// Data should be a Uint8Array of r, g, b pairs.
export function sendLightData(data: Uint8Array) {
    websocket.send(data);
}

function updateStatus() {
    const data = currentData;

    statusMessage.set(`
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
Global CPU: ${Math.round(currentSystemData.global_cpu * 10) / 10}%<br>
Free memory: ${Math.round(currentSystemData.available_memory / 1024 / 1024)}MB / ${Math.round(currentSystemData.total_memory / 1024 / 1024)}MB (${Math.round(currentSystemData.available_memory / currentSystemData.total_memory * 100)}%)<br>
Used swap: ${Math.round(currentSystemData.used_swap / 1024 / 1024 * 10) / 10}MB
`);
}