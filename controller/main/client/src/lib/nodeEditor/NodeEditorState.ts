import { writable, type Writable } from 'svelte/store';
import type { CameraState, EdgeData, NodeData } from './NodeTypes';

export default class NodeEditorState {
    nodes: Writable<NodeData[]>;
    edges: Writable<EdgeData[]>;
    camera: Writable<CameraState>;

    getNode(id: string): Writable<NodeData> {
        // This is super hacky, but whatever...
        return {
            subscribe: (run) => {
                const unsubscribe = this.nodes.subscribe((nodes) => {
                    const node = nodes.find((n) => n.id === id);
                    if(node) run(node);
                    else throw new Error(`Node with id ${id} not found`)
                });
                return unsubscribe;
            },
            set: (newNode: NodeData) => {
                this.nodes.update((nodes) => {
                    const index = nodes.findIndex((n) => n.id === id);
                    if(index !== -1) nodes[index] = newNode;
                    return nodes;
                });
            },
            update: (updater: (node: NodeData) => NodeData) => {
                this.nodes.update((nodes) => {
                    const index = nodes.findIndex((n) => n.id === id);
                    if(index !== -1) nodes[index] = updater(nodes[index]);
                    return nodes;
                });
            }
        };
    }

    constructor() {
        this.nodes = writable<NodeData[]>([
            { id: 'node1', x: 100, y: 100, width: 150, label: 'Input A', inputs: [], outputs: ['Value'], zIndex: 0 },
            { id: 'node2', x: 100, y: 300, width: 150, label: 'Input B', inputs: [], outputs: ['Value'], zIndex: 1 },
            { id: 'node3', x: 300, y: 200, width: 150, label: 'Add', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 2 },
            { id: 'node4', x: 500, y: 100, width: 150, label: 'Multiply', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 3 },
            { id: 'node5', x: 500, y: 300, width: 150, label: 'Subtract', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 4 },
            { id: 'node6', x: 700, y: 200, width: 150, label: 'Divide', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 5 },
            { id: 'node7', x: 900, y: 200, width: 150, label: 'Output', inputs: ['Value'], outputs: [], zIndex: 6 },
            { id: 'node8', x: 300, y: 400, width: 150, label: 'Constant C', inputs: [], outputs: ['Value'], zIndex: 7 },
            { id: 'node9', x: 500, y: 500, width: 150, label: 'Power', inputs: ['Base', 'Exponent'], outputs: ['Result'], zIndex: 8 },
            { id: 'node10', x: 700, y: 400, width: 150, label: 'Modulo', inputs: ['A', 'B'], outputs: ['Result'], zIndex: 9 },
            { id: 'node11', x: 900, y: 400, width: 150, label: 'Output 2', inputs: ['Value'], outputs: [], zIndex: 10 },
            { id: 'node12', x: 100, y: 500, width: 150, label: 'Input D', inputs: [], outputs: ['Value'], zIndex: 11 },
            { id: 'node13', x: 300, y: 600, width: 150, label: 'Logarithm', inputs: ['Value'], outputs: ['Result'], zIndex: 12 },
            { id: 'node14', x: 500, y: 700, width: 150, label: 'Square Root', inputs: ['Value'], outputs: ['Result'], zIndex: 13 },
            { id: 'node15', x: 700, y: 600, width: 150, label: 'Sin', inputs: ['Angle'], outputs: ['Result'], zIndex: 14 },
            { id: 'node16', x: 900, y: 600, width: 150, label: 'Cos', inputs: ['Angle'], outputs: ['Result'], zIndex: 15 },
            { id: 'node17', x: 1100, y: 200, width: 150, label: 'Final Output', inputs: ['Value'], outputs: [], zIndex: 16 },
            { id: 'node18', x: 1100, y: 400, width: 150, label: 'Final Output 2', inputs: ['Value'], outputs: [], zIndex: 17 },
            { id: 'node19', x: 1100, y: 600, width: 150, label: 'Final Output 3', inputs: ['Value'], outputs: [], zIndex: 18 }
        ]);

        this.edges = writable<EdgeData[]>([
            { id: 'edge1', from: { nodeId: 'node1', outputIndex: 0 }, to: { nodeId: 'node3', inputIndex: 0 } },
            { id: 'edge2', from: { nodeId: 'node2', outputIndex: 0 }, to: { nodeId: 'node3', inputIndex: 1 } },
            { id: 'edge3', from: { nodeId: 'node3', outputIndex: 0 }, to: { nodeId: 'node4', inputIndex: 0 } },
            { id: 'edge4', from: { nodeId: 'node8', outputIndex: 0 }, to: { nodeId: 'node4', inputIndex: 1 } },
            { id: 'edge5', from: { nodeId: 'node4', outputIndex: 0 }, to: { nodeId: 'node6', inputIndex: 0 } },
            { id: 'edge6', from: { nodeId: 'node5', outputIndex: 0 }, to: { nodeId: 'node6', inputIndex: 1 } },
            { id: 'edge7', from: { nodeId: 'node6', outputIndex: 0 }, to: { nodeId: 'node7', inputIndex: 0 } },
            { id: 'edge8', from: { nodeId: 'node9', outputIndex: 0 }, to: { nodeId: 'node10', inputIndex: 0 } },
            { id: 'edge9', from: { nodeId: 'node10', outputIndex: 0 }, to: { nodeId: 'node11', inputIndex: 0 } },
            { id: 'edge10', from: { nodeId: 'node12', outputIndex: 0 }, to: { nodeId: 'node13', inputIndex: 0 } },
            { id: 'edge11', from: { nodeId: 'node13', outputIndex: 0 }, to: { nodeId: 'node14', inputIndex: 0 } },
            { id: 'edge12', from: { nodeId: 'node14', outputIndex: 0 }, to: { nodeId: 'node15', inputIndex: 0 } },
            { id: 'edge13', from: { nodeId: 'node15', outputIndex: 0 }, to: { nodeId: 'node16', inputIndex: 0 } },
            { id: 'edge14', from: { nodeId: 'node16', outputIndex: 0 }, to: { nodeId: 'node17', inputIndex: 0 } },
            { id: 'edgeidk', from: { nodeId: 'node8', outputIndex: 0 }, to: { nodeId: 'node9', inputIndex: 0 } },
            { id: 'edgeidk2', from: { nodeId: 'node8', outputIndex: 0 }, to: { nodeId: 'node5', inputIndex: 0 } },
            { id: 'edgeidk3', from: { nodeId: 'node8', outputIndex: 0 }, to: { nodeId: 'node5', inputIndex: 1 } }
        ]);

        this.camera = writable<CameraState>({
            center: { x: 0, y: 0 },
            zoom: 1
        });
    }
}