import { get, writable, type Writable } from 'svelte/store';
import type { CameraState, EdgeData, EdgeID, NodeData, NodeID, SelectionState } from './NodeTypes';

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
        this.nodes = writable(new Map(([
            { id: 'node1' as NodeID, x: 100, y: 100, width: 150, label: 'Input A', inputs: [], outputs: ['Value'], zIndex: 0 },
            { id: 'node2' as NodeID, x: 100, y: 300, width: 150, label: 'Input B', inputs: [], outputs: ['Value'], zIndex: 1 },
            { id: 'node3' as NodeID, x: 300, y: 200, width: 150, label: 'Add', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 2 },
            { id: 'node4' as NodeID, x: 500, y: 100, width: 150, label: 'Multiply', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 3 },
            { id: 'node5' as NodeID, x: 500, y: 300, width: 150, label: 'Subtract', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 4 },
            { id: 'node6' as NodeID, x: 700, y: 200, width: 150, label: 'Divide', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 5 },
            { id: 'node7' as NodeID, x: 900, y: 200, width: 150, label: 'Output', inputs: ['Value'], outputs: [], zIndex: 6 },
            { id: 'node8' as NodeID, x: 300, y: 400, width: 150, label: 'Constants C', inputs: [], outputs: ['One', 'Two'], zIndex: 7 },
            { id: 'node9' as NodeID, x: 500, y: 500, width: 150, label: 'Power', inputs: ['Base', 'Exponent'], outputs: ['Result'], zIndex: 8 },
            { id: 'node10' as NodeID, x: 700, y: 400, width: 150, label: 'Modulo', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 9 },
            { id: 'node11' as NodeID, x: 900, y: 400, width: 150, label: 'Output 2', inputs: ['Value'], outputs: [], zIndex: 10 },
            { id: 'node12' as NodeID, x: 100, y: 500, width: 150, label: 'Input D', inputs: [], outputs: ['Value'], zIndex: 11 },
            { id: 'node13' as NodeID, x: 300, y: 600, width: 150, label: 'Logarithm', inputs: ['Value'], outputs: ['Result'], zIndex: 12 },
            { id: 'node14' as NodeID, x: 500, y: 700, width: 150, label: 'Square Root', inputs: ['Value'], outputs: ['Result'], zIndex: 13 },
            { id: 'node15' as NodeID, x: 700, y: 600, width: 150, label: 'Add', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 14 },
            { id: 'node16' as NodeID, x: 900, y: 600, width: 150, label: 'Cos', inputs: ['Angle'], outputs: ['Result'], zIndex: 15 },
            { id: 'node17' as NodeID, x: 1100, y: 200, width: 150, label: 'Final Output', inputs: ['Value'], outputs: [], zIndex: 16 }
        ] satisfies NodeData[]).map((n) => [n.id, n])));

        this.edges = writable<EdgeData[]>([
            { id: 'edge1' as EdgeID, from: { nodeId: 'node1' as NodeID, outputIndex: 0 }, to: { nodeId: 'node3' as NodeID, inputIndex: 0 } },
            { id: 'edge2' as EdgeID, from: { nodeId: 'node2' as NodeID, outputIndex: 0 }, to: { nodeId: 'node3' as NodeID, inputIndex: 1 } },
            { id: 'edge3' as EdgeID, from: { nodeId: 'node3' as NodeID, outputIndex: 0 }, to: { nodeId: 'node4' as NodeID, inputIndex: 0 } },
            { id: 'edge4' as EdgeID, from: { nodeId: 'node8' as NodeID, outputIndex: 0 }, to: { nodeId: 'node4' as NodeID, inputIndex: 1 } },
            { id: 'edge5' as EdgeID, from: { nodeId: 'node4' as NodeID, outputIndex: 0 }, to: { nodeId: 'node6' as NodeID, inputIndex: 0 } },
            { id: 'edge6' as EdgeID, from: { nodeId: 'node5' as NodeID, outputIndex: 0 }, to: { nodeId: 'node6' as NodeID, inputIndex: 1 } },
            { id: 'edge7' as EdgeID, from: { nodeId: 'node6' as NodeID, outputIndex: 0 }, to: { nodeId: 'node7' as NodeID, inputIndex: 0 } },
            { id: 'edge8' as EdgeID, from: { nodeId: 'node9' as NodeID, outputIndex: 0 }, to: { nodeId: 'node10' as NodeID, inputIndex: 0 } },
            { id: 'edge9' as EdgeID, from: { nodeId: 'node10' as NodeID, outputIndex: 0 }, to: { nodeId: 'node11' as NodeID, inputIndex: 0 } },
            { id: 'edge10' as EdgeID, from: { nodeId: 'node12' as NodeID, outputIndex: 0 }, to: { nodeId: 'node13' as NodeID, inputIndex: 0 } },
            { id: 'edge11' as EdgeID, from: { nodeId: 'node13' as NodeID, outputIndex: 0 }, to: { nodeId: 'node14' as NodeID, inputIndex: 0 } },
            { id: 'edge12' as EdgeID, from: { nodeId: 'node14' as NodeID, outputIndex: 0 }, to: { nodeId: 'node15' as NodeID, inputIndex: 1 } },
            { id: 'edge13' as EdgeID, from: { nodeId: 'node15' as NodeID, outputIndex: 0 }, to: { nodeId: 'node16' as NodeID, inputIndex: 0 } },
            { id: 'edge14' as EdgeID, from: { nodeId: 'node16' as NodeID, outputIndex: 0 }, to: { nodeId: 'node17' as NodeID, inputIndex: 0 } },
            { id: 'edgeidk' as EdgeID, from: { nodeId: 'node8' as NodeID, outputIndex: 0 }, to: { nodeId: 'node9' as NodeID, inputIndex: 0 } },
            { id: 'edgeidk2' as EdgeID, from: { nodeId: 'node8' as NodeID, outputIndex: 0 }, to: { nodeId: 'node5' as NodeID, inputIndex: 0 } },
            { id: 'edgeidk3' as EdgeID, from: { nodeId: 'node8' as NodeID, outputIndex: 0 }, to: { nodeId: 'node5' as NodeID, inputIndex: 1 } },
            { id: 'edgeidk4' as EdgeID, from: { nodeId: 'node8' as NodeID, outputIndex: 1 }, to: { nodeId: 'node15' as NodeID, inputIndex: 0 } }
        ]);

        this.camera = writable<CameraState>({
            center: { x: 0, y: 0 },
            zoom: 1
        });

        this.selection = writable<SelectionState>({
            nodes: new Set<NodeID>(),
            activeNode: null
        });
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
    }
}