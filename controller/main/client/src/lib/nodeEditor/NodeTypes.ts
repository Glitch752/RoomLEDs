// VERY temporary for testing rendering

import { nodeVariantInfo, type NodeVariantDescriptor, type NodeVariant, type NodeDataValue } from "./NodeVariants";

export type NodeID = string & { readonly __brand: unique symbol };
export type EdgeID = string & { readonly __brand: unique symbol };


export type SerializedNodeEditorState = {
    version: 1;
    nodes: SerializedNodeData[];
    edges: EdgeData[];
    camera: CameraState;
};

export type SerializedNodeData = {
    id: NodeID;
    variant: NodeVariant;
    zonePartner: NodeID | null;
    x: number;
    y: number;
    width: number;
    zIndex: number;
    dataValues: {
        [key: string]: NodeDataValue;
    };
};


export class NodeData {
    id: NodeID;
    variant: NodeVariant;

    get variantInfo(): NodeVariantDescriptor {
        return nodeVariantInfo[this.variant];
    }

    /**
     * For a zone start, the ID of the corresponding zone end node.
     * For a zone end, the ID of the corresponding zone start node.
     * For other nodes, null.
     */
    zonePartner: NodeID | null;

    x: number;
    y: number;
    width: number;
    zIndex: number; // TODO: make this actually work

    // y positions relative to the node, in canvas pixels (not screen pixels)
    inputPositionCache: number[] = [];
    // y positions relative to the node, in canvas pixels (not screen pixels)
    outputPositionCache: number[] = [];

    dataValues: {
        [key: string]: number | string | boolean;
    };

    constructor(data: SerializedNodeData) {
        this.id = data.id;
        this.variant = data.variant;
        this.zonePartner = data.zonePartner;
        this.x = data.x;
        this.y = data.y;
        this.width = data.width;
        this.zIndex = data.zIndex;
        this.dataValues = data.dataValues;
    }
};

export type EdgeData = {
    id: EdgeID;
    from: {
        nodeId: NodeID;
        outputIndex: number;
    };
    to: {
        nodeId: NodeID;
        inputIndex: number;
    };
};

export type CameraState = {
    center: { x: number; y: number };
    zoom: number;
};

export type SelectionState = {
    nodes: Set<NodeID>;
    // The primary active node. guarenteed to be present in `nodes`.
    // It's possible to have a nonempty selection without an active node,
    // e.g. when the user clicks and drags to select multiple nodes and
    // doesn't already have one of them selected.
    activeNode: NodeID | null;
};

export type MarqueeState = {
    active: boolean;
    startX: number;
    startY: number;
    endX: number;
    endY: number;
};