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
};

export type SelectionState = {
    nodes: Set<string>;
    // The primary active node. guarenteed to be present in `nodes`.
    // It's possible to have a nonempty selection without an active node,
    // e.g. when the user clicks and drags to select multiple nodes and
    // doesn't already have one of them selected.
    activeNode: string | null;
};

export type MarqueeState = {
    active: boolean;
    startX: number;
    startY: number;
    endX: number;
    endY: number;
};