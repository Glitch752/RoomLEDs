// VERY temporary for testing rendering

export type NodeData = {
    id: string;
    x: number;
    y: number;
    width: number;
    label: string;
    inputs: string[];
    outputs: string[];

    // y positions relative to the node, in canvas pixels (not screen pixels)
    inputPositionCache?: number[];
    // y positions relative to the node, in canvas pixels (not screen pixels)
    outputPositionCache?: number[];

    zIndex: number;
};

export type EdgeData = {
    id: string;
    from: {
        nodeId: string;
        outputIndex: number;
    };
    to: {
        nodeId: string;
        inputIndex: number;
    };
};

export type CameraState = {
    center: { x: number; y: number };
    zoom: number;
}