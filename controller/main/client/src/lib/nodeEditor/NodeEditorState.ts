import { get, writable, type Writable } from 'svelte/store';
import { tick } from 'svelte';
import { NodeData, type CameraState, type EdgeData, type EdgeID, type NodeID, type SelectionState, type SerializedNodeData, type SerializedNodeEditorState } from './NodeTypes';
import { NodeVariant } from './NodeVariants';

export type DraggingEdgeData = {
    type: 'to',
    toNodeId: NodeID;
    toInputIndex: number;
    from: {
        type: 'point';
        x: number;
        y: number;
    } | {
        type: 'nodeOutput';
        nodeId: NodeID;
        outputIndex: number;
    }
} | {
    type: 'from',
    fromNodeId: NodeID;
    fromOutputIndex: number;
    to: {
        type: 'point';
        x: number;
        y: number;
    } | {
        type: 'nodeInput';
        nodeId: NodeID;
        inputIndex: number;
    }
} | null;

export type NodeEditMode = {
    type: "none",
} | {
    type: "drag-move",
    didMouseMove: boolean
} | {
    type: "keyboard-move",
    end: () => void
};

export default class NodeEditorState {
    nodes: Writable<Map<NodeID, NodeData>>;
    edges: Writable<EdgeData[]>;
    camera: Writable<CameraState>;

    editorElement: HTMLElement | null = null;
    get editorRect() {
        return this.editorElement?.getBoundingClientRect() ?? { left: 0, top: 0, right: 0, bottom: 0, width: 0, height: 0 };
    }

    draggingEdge: Writable<DraggingEdgeData> = writable(null);

    selection: Writable<SelectionState>;
    editMode: Writable<NodeEditMode> = writable({ type: "none" });

    private get currentEdges(): EdgeData[] {
        return get(this.edges);
    }
    private get currentNodes(): MapIterator<NodeData> {
        return get(this.nodes).values();
    }

    getNode(id: NodeID): Writable<NodeData> {
        // This is super hacky, but whatever...
        return {
            subscribe: (run) => {
                const unsubscribe = this.nodes.subscribe((nodes) => {
                    const node = nodes.get(id);
                    if(node) run(node);
                });
                return unsubscribe;
            },
            set: (newNode: NodeData) => {
                this.nodes.update((nodes) => {
                    nodes.set(id, newNode);
                    return nodes;
                });
            },
            update: (updater: (node: NodeData) => NodeData) => {
                this.nodes.update((nodes) => {
                    const existingNode = nodes.get(id);
                    if(existingNode) {
                        nodes.set(id, updater(existingNode));
                    }
                    return nodes;
                });
            }
        };
    }

    getNodeValue(id: string): NodeData {
        const node = this.currentNodes.find((n) => n.id === id);
        if(node) return node;
        throw new Error(`Node with id ${id} not found`);
    }

    getEdge(id: string): Writable<EdgeData> {
        // This is super hacky, but whatever...
        return {
            subscribe: (run) => {
                const unsubscribe = this.edges.subscribe((edges) => {
                    const edge = edges.find((e) => e.id === id);
                    if(edge) run(edge);
                });
                return unsubscribe;
            },
            set: (newEdge: EdgeData) => {
                this.edges.update((edges) => {
                    const index = edges.findIndex((e) => e.id === id);
                    if(index !== -1) edges[index] = newEdge;
                    return edges;
                });
            },
            update: (updater: (edge: EdgeData) => EdgeData) => {
                this.edges.update((edges) => {
                    const index = edges.findIndex((e) => e.id === id);
                    if(index !== -1) edges[index] = updater(edges[index]);
                    return edges;
                });
            }
        };
    }

    getEdgeValue(id: string): EdgeData {
        const edge = this.currentEdges.find((e) => e.id === id);
        if(edge) return edge;
        throw new Error(`Edge with id ${id} not found`);
    }

    getInputEdge(nodeId: string, inputIndex: number): string | null {
        const edge = this.currentEdges.find(
            (e) => e.to.nodeId === nodeId && e.to.inputIndex === inputIndex
        )?.id;
        if(edge) return edge;
        return null;
    }

    removeEdge(id: string): void {
        this.edges.update((edges) => edges.filter((e) => e.id !== id));
    }

    addEdge(edge: EdgeData): void {
        this.edges.update((edges) => [...edges, edge]);
    }

    constructor() {
        this.nodes = writable(new Map());
        this.edges = writable<EdgeData[]>([]);

        this.generateTestGrid(10);

        this.camera = writable<CameraState>({
            center: { x: 0, y: 0 },
            zoom: 1
        });

        this.selection = writable<SelectionState>({
            nodes: new Set<NodeID>(),
            activeNode: null
        });
    }

    serialize(): SerializedNodeEditorState {
        const nodesArray: SerializedNodeData[] = [];
        get(this.nodes).forEach((node) => {
            const { inputPositionCache, outputPositionCache, ...rest } = node;
            nodesArray.push(rest);
        });
        
        return {
            version: 1,
            nodes: nodesArray,
            edges: get(this.edges),
            camera: get(this.camera)
        };
    }

    async deserialize(data: SerializedNodeEditorState): Promise<void> {
        this.edges.set([]);
        this.nodes.set(new Map());

        // Tell svelte to wait a tick to ensure reactivity
        await tick();

        const nodesMap: Map<NodeID, NodeData> = new Map();
        data.nodes.forEach((node) => {
            nodesMap.set(node.id, new NodeData(node));
        });
        this.nodes.set(nodesMap);
        this.edges.set(data.edges);
        this.camera.set(data.camera);
    }



    // spaghetti, yum 🍝
    // only for testing! this isn't made to be good code
    generateTestGrid(n: number) {
        this.nodes.set(new Map());
        this.edges.set([]);

        const nodes = new Map<NodeID, NodeData>();
        const edges: EdgeData[] = [];
        let edgeId = 0;

        for(let row = 0; row < n; row++) {
            for(let col = row; col < n; col++) {
                const id = `node-${row}-${col}` as NodeID;
                const middleNode = col !== 0 && col !== n - 1;
                nodes.set(id, new NodeData({
                    id,
                    variant: col === 0 ? NodeVariant.InputNumber : (
                        col === n - 1 ? NodeVariant.OutputNumber : NodeVariant.UnaryNumberOperation
                    ),
                    x: col * 300 + Math.floor(Math.random() * 100 - 50),
                    y: row * 180 + Math.floor(Math.random() * 100 - 50),
                    width: middleNode ? 200 : 150,
                    zonePartner: null,
                    dataValues: middleNode ? {
                        operation: 'sqrt'
                    } : {},
                    zIndex: 0
                }));
            }
        }

        const getNodeId = (row: number, col: number) => `node-${row}-${col}` as NodeID;

        const addEdgeIfValid = (fromRow: number, fromCol: number, toRow: number, toCol: number) => {
            if(fromCol < 0 || fromCol >= n || toCol < 0 || toCol >= n) return;
            const fromId = getNodeId(fromRow, fromCol);
            const toId = getNodeId(toRow, toCol);
            if(!nodes.has(fromId) || !nodes.has(toId)) return;

            // Check if the to node already has an edge on that input
            const existingEdge = edges.find((e) => e.to.nodeId === toId && e.to.inputIndex === 0);
            if(existingEdge) return;
            
            edges.push({
                id: `edge-${edgeId++}`,
                from: { nodeId: fromId, outputIndex: 0 },
                to: { nodeId: toId, inputIndex: 0 },
            } as EdgeData);
        };

        // For each column, pick random nodes in the previous column
        // to make edges with
        for(let col = 1; col < n; col++) {
            const possibleFromRows = [];
            for(let row = 0; row < n; row++) {
                if(nodes.has(getNodeId(row, col - 1))) {
                    possibleFromRows.push(row);
                }
            }
            
            for(let row = 0; row < n; row++) {
                const fromRow = possibleFromRows[Math.floor(Math.random() * possibleFromRows.length)];
                addEdgeIfValid(fromRow, col - 1, row, col);
            }
        }

        // Remove nodes with no outputs
        let removed = true;
        while(removed) {
            removed = false;
            nodes.forEach((node, nodeId) => {
                if(node.variantInfo.outputs.length !== 0) {
                    const hasOutgoingEdge = edges.some((edge) => edge.from.nodeId === nodeId);
                    if(!hasOutgoingEdge) {
                        nodes.delete(nodeId);

                        // Delete edges connected to this node
                        for(let i = edges.length - 1; i >= 0; i--) {
                            if(edges[i].from.nodeId === nodeId || edges[i].to.nodeId === nodeId) {
                                edges.splice(i, 1);
                            }
                        }

                        removed = true;
                    }
                }
            });
        }

        this.nodes.set(nodes);
        this.edges.set(edges);
    }

    clientToCanvas(x: number, y: number): { x: number; y: number } {
        const camera = get(this.camera);
        const rect = this.editorRect;
        const canvasX = (x - rect.left - rect.width / 2) / camera.zoom + camera.center.x;
        const canvasY = (y - rect.top - rect.height / 2) / camera.zoom + camera.center.y;
        return { x: canvasX, y: canvasY };
    }
    mouseEventToCanvasPos(e: MouseEvent): { x: number; y: number } {
        return this.clientToCanvas(e.pageX, e.pageY);
    }

    onkeydown(e: KeyboardEvent) {
        if(e.key === 'Delete') {
            const selection = get(this.selection);
            this.selection.set({ nodes: new Set<NodeID>(), activeNode: null } );

            // first delete all edges connected to selected nodes
            this.edges.update((edges) => edges.filter((edge) => {
                return (
                    !selection.nodes.has(edge.from.nodeId) &&
                    !selection.nodes.has(edge.to.nodeId)
                );
            }));

            // delete the selected nodes
            this.nodes.update((nodes) => {
                selection.nodes.forEach((nodeId) => {
                    nodes.delete(nodeId);
                });
                return nodes;
            });
        }

        if(e.key.toLowerCase() === 'g') {
            // Start keyboard move mode
            const editMode = get(this.editMode);
            if(editMode.type === "none") {
                this.editMode.set({
                    type: "keyboard-move",
                    end: () => {
                        this.editMode.set({ type: "none" });
                    }
                });
            } else if(editMode.type === "keyboard-move") {
                editMode.end();
            }
        }
    }

    onmousemove(e: MouseEvent) {
        const editMode = get(this.editMode);
        if(editMode.type !== "drag-move" && editMode.type !== "keyboard-move") return;

        const camera = get(this.camera);

        if(editMode.type === "drag-move" && !editMode.didMouseMove) this.editMode.set({
            ...editMode,
            didMouseMove: true
        });

        // Drag every selected node
        for(const nodeId of get(this.selection).nodes) {
            const n = this.getNode(nodeId);
            if(!n) continue;

            n.update(n => {
                n.x += e.movementX / camera.zoom;
                n.y += e.movementY / camera.zoom;
                return n;
            });
        }
    }

    onmousedown(e: MouseEvent) {
        const editMode = get(this.editMode);
        if(editMode.type === "none") return;

        this.editMode.set({ type: "none" });
        e.preventDefault();
        e.stopPropagation();
    }
}