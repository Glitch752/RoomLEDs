import { get, writable, type Writable } from 'svelte/store';
import { tick } from 'svelte';
import { NodeData, type CameraState, type EdgeData, type EdgeID, type NodeID, type SelectionState, type SerializedNodeData, type SerializedDocumentState } from './NodeTypes';

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

export default class NEDocumentState {
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
        
        this.camera = writable<CameraState>({
            center: { x: 0, y: 0 },
            zoom: 1
        });

        this.deserialize(JSON.parse('{"version":1,"nodes":[{"id":"node-0-0","variant":0,"zonePartner":null,"x":-141.95570880296808,"y":524.6917003928421,"width":150,"zIndex":0,"dataValues":{}},{"id":"node-0-4","variant":29,"zonePartner":null,"x":1228.8927871089236,"y":180.52683273147554,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-0-8","variant":29,"zonePartner":null,"x":3202.791470447396,"y":-59.333680913536945,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-0-9","variant":6,"zonePartner":null,"x":3216.0623077600403,"y":1029.5343535743855,"width":150,"zIndex":0,"dataValues":{}},{"id":"node-1-1","variant":29,"zonePartner":null,"x":184.5170995769578,"y":502.5540743219631,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-1-2","variant":29,"zonePartner":null,"x":521.7978986458531,"y":695.9724994617342,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-1-5","variant":29,"zonePartner":null,"x":1578.9487796338549,"y":692.9382846076647,"width":200,"zIndex":0,"dataValues":{"operation":"negate"}},{"id":"node-1-6","variant":29,"zonePartner":null,"x":1856.4041478040954,"y":177.18014964392518,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-1-8","variant":29,"zonePartner":null,"x":3187.275806912383,"y":110.28317903885028,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-1-9","variant":6,"zonePartner":null,"x":3185.5987590607815,"y":268.9530116831554,"width":150,"zIndex":0,"dataValues":{}},{"id":"node-2-2","variant":29,"zonePartner":null,"x":614.7091088879855,"y":207.47083149123236,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-2-3","variant":29,"zonePartner":null,"x":924.4724109127977,"y":177.83129122152042,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-2-4","variant":29,"zonePartner":null,"x":1233.7396406707428,"y":933.525927680031,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-2-5","variant":29,"zonePartner":null,"x":1850.4616478143334,"y":1116.7727655654196,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-2-9","variant":6,"zonePartner":null,"x":3310.2892334788166,"y":1182.2155627316256,"width":150,"zIndex":0,"dataValues":{}},{"id":"node-3-3","variant":29,"zonePartner":null,"x":824.4373995881461,"y":716.1521266288267,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-3-4","variant":29,"zonePartner":null,"x":1257.6377391953063,"y":706.4969273990024,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-3-5","variant":29,"zonePartner":null,"x":1511.832242265747,"y":177.25126623361012,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-3-7","variant":29,"zonePartner":null,"x":2247.6275422652047,"y":667.6456768047979,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-3-8","variant":29,"zonePartner":null,"x":2840.867760390102,"y":1024.8388766594394,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-3-9","variant":6,"zonePartner":null,"x":3338.483268861375,"y":1299.6704340560123,"width":150,"zIndex":0,"dataValues":{}},{"id":"node-4-8","variant":29,"zonePartner":null,"x":2546.288536948397,"y":675.6999200554962,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-4-9","variant":6,"zonePartner":null,"x":3320.533164853259,"y":1455.9281897435703,"width":150,"zIndex":0,"dataValues":{}},{"id":"node-5-6","variant":29,"zonePartner":null,"x":1929.3710335831095,"y":677.9503599518284,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-5-7","variant":29,"zonePartner":null,"x":2477.1156932130552,"y":1106.772713394692,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-5-9","variant":6,"zonePartner":null,"x":2886.3956988670093,"y":676.6113717870262,"width":150,"zIndex":0,"dataValues":{}},{"id":"node-6-6","variant":29,"zonePartner":null,"x":2153.049195701635,"y":1150.5809525694533,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-6-7","variant":29,"zonePartner":null,"x":2209.1509438858275,"y":186.51164984444716,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-6-8","variant":29,"zonePartner":null,"x":2589.9075425329142,"y":313.5574073290674,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-6-9","variant":6,"zonePartner":null,"x":3196.234048403714,"y":389.99894220123707,"width":150,"zIndex":0,"dataValues":{}},{"id":"node-7-8","variant":29,"zonePartner":null,"x":2901.9016327494587,"y":1189.8276828842047,"width":200,"zIndex":0,"dataValues":{"operation":"sqrt"}},{"id":"node-7-9","variant":6,"zonePartner":null,"x":3496.9417382786146,"y":124.47877955754083,"width":150,"zIndex":0,"dataValues":{}},{"id":"node-8-9","variant":6,"zonePartner":null,"x":3203.001110916343,"y":500.96003033314366,"width":150,"zIndex":0,"dataValues":{}},{"id":"node-9-9","variant":6,"zonePartner":null,"x":3499.1539647213935,"y":-56.980152102787606,"width":150,"zIndex":0,"dataValues":{}}],"edges":[{"id":"edge-1","from":{"nodeId":"node-0-0","outputIndex":0},"to":{"nodeId":"node-1-1","inputIndex":0}},{"id":"edge-3","from":{"nodeId":"node-1-1","outputIndex":0},"to":{"nodeId":"node-1-2","inputIndex":0}},{"id":"edge-4","from":{"nodeId":"node-1-1","outputIndex":0},"to":{"nodeId":"node-2-2","inputIndex":0}},{"id":"edge-7","from":{"nodeId":"node-2-2","outputIndex":0},"to":{"nodeId":"node-2-3","inputIndex":0}},{"id":"edge-8","from":{"nodeId":"node-1-2","outputIndex":0},"to":{"nodeId":"node-3-3","inputIndex":0}},{"id":"edge-9","from":{"nodeId":"node-2-3","outputIndex":0},"to":{"nodeId":"node-0-4","inputIndex":0}},{"id":"edge-11","from":{"nodeId":"node-3-3","outputIndex":0},"to":{"nodeId":"node-2-4","inputIndex":0}},{"id":"edge-12","from":{"nodeId":"node-3-3","outputIndex":0},"to":{"nodeId":"node-3-4","inputIndex":0}},{"id":"edge-15","from":{"nodeId":"node-3-4","outputIndex":0},"to":{"nodeId":"node-1-5","inputIndex":0}},{"id":"edge-16","from":{"nodeId":"node-2-4","outputIndex":0},"to":{"nodeId":"node-2-5","inputIndex":0}},{"id":"edge-17","from":{"nodeId":"node-0-4","outputIndex":0},"to":{"nodeId":"node-3-5","inputIndex":0}},{"id":"edge-21","from":{"nodeId":"node-3-5","outputIndex":0},"to":{"nodeId":"node-1-6","inputIndex":0}},{"id":"edge-25","from":{"nodeId":"node-1-5","outputIndex":0},"to":{"nodeId":"node-5-6","inputIndex":0}},{"id":"edge-26","from":{"nodeId":"node-2-5","outputIndex":0},"to":{"nodeId":"node-6-6","inputIndex":0}},{"id":"edge-30","from":{"nodeId":"node-5-6","outputIndex":0},"to":{"nodeId":"node-3-7","inputIndex":0}},{"id":"edge-32","from":{"nodeId":"node-6-6","outputIndex":0},"to":{"nodeId":"node-5-7","inputIndex":0}},{"id":"edge-33","from":{"nodeId":"node-1-6","outputIndex":0},"to":{"nodeId":"node-6-7","inputIndex":0}},{"id":"edge-35","from":{"nodeId":"node-6-7","outputIndex":0},"to":{"nodeId":"node-0-8","inputIndex":0}},{"id":"edge-36","from":{"nodeId":"node-6-7","outputIndex":0},"to":{"nodeId":"node-1-8","inputIndex":0}},{"id":"edge-38","from":{"nodeId":"node-5-7","outputIndex":0},"to":{"nodeId":"node-3-8","inputIndex":0}},{"id":"edge-39","from":{"nodeId":"node-3-7","outputIndex":0},"to":{"nodeId":"node-4-8","inputIndex":0}},{"id":"edge-41","from":{"nodeId":"node-6-7","outputIndex":0},"to":{"nodeId":"node-6-8","inputIndex":0}},{"id":"edge-42","from":{"nodeId":"node-5-7","outputIndex":0},"to":{"nodeId":"node-7-8","inputIndex":0}},{"id":"edge-44","from":{"nodeId":"node-3-8","outputIndex":0},"to":{"nodeId":"node-0-9","inputIndex":0}},{"id":"edge-45","from":{"nodeId":"node-6-8","outputIndex":0},"to":{"nodeId":"node-1-9","inputIndex":0}},{"id":"edge-46","from":{"nodeId":"node-7-8","outputIndex":0},"to":{"nodeId":"node-2-9","inputIndex":0}},{"id":"edge-47","from":{"nodeId":"node-7-8","outputIndex":0},"to":{"nodeId":"node-3-9","inputIndex":0}},{"id":"edge-48","from":{"nodeId":"node-7-8","outputIndex":0},"to":{"nodeId":"node-4-9","inputIndex":0}},{"id":"edge-49","from":{"nodeId":"node-4-8","outputIndex":0},"to":{"nodeId":"node-5-9","inputIndex":0}},{"id":"edge-50","from":{"nodeId":"node-6-8","outputIndex":0},"to":{"nodeId":"node-6-9","inputIndex":0}},{"id":"edge-51","from":{"nodeId":"node-1-8","outputIndex":0},"to":{"nodeId":"node-7-9","inputIndex":0}},{"id":"edge-52","from":{"nodeId":"node-6-8","outputIndex":0},"to":{"nodeId":"node-8-9","inputIndex":0}},{"id":"edge-53","from":{"nodeId":"node-0-8","outputIndex":0},"to":{"nodeId":"node-9-9","inputIndex":0}}],"camera":{"center":{"x":2052.088631599336,"y":733.8670938596996},"zoom":0.27363781062882725}}'));

        this.selection = writable<SelectionState>({
            nodes: new Set<NodeID>(),
            activeNode: null
        });
    }

    serialize(): SerializedDocumentState {
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

    async deserialize(data: SerializedDocumentState): Promise<void> {
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

    private mouseDownHandlers: {
        once: boolean,
        handler: (e: MouseEvent) => void
    }[] = [];
    private mouseMoveHandlers: {
        removeWhenMouseUp: boolean,
        handler: (e: MouseEvent) => void
    }[] = [];
    private mouseUpHandlers: {
        once: boolean,
        handler: (e: MouseEvent) => void
    }[] = [];

    public handleMouseDown(handler: (e: MouseEvent) => void): () => void {
        this.mouseDownHandlers.push({
            once: false,
            handler
        });
        return () => {
            this.mouseDownHandlers = this.mouseDownHandlers.filter(h => h.handler !== handler);
        };
    }
    public handleMouseMoveUntilMouseUp(handler: (e: MouseEvent) => void) {
        this.mouseMoveHandlers.push({
            removeWhenMouseUp: true,
            handler
        });
    }
    public handleMouseUpOnce(handler: (e: MouseEvent) => void) {
        this.mouseUpHandlers.push({
            once: true,
            handler
        });
    }

    public onmousemove(e: MouseEvent) {
        for(const handler of this.mouseMoveHandlers) {
            handler.handler(e);
        }

        const editMode = get(this.editMode);
        if(editMode.type !== "drag-move" && editMode.type !== "keyboard-move") return;

        const camera = get(this.camera);

        if(editMode.type === "drag-move" && !editMode.didMouseMove) this.editMode.set({
            ...editMode,
            didMouseMove: true
        });


        const moveSpeed = e.shiftKey ? 0.1 : 1;

        // Drag every selected node
        for(const nodeId of get(this.selection).nodes) {
            const n = this.getNode(nodeId);
            if(!n) continue;

            n.update(n => {
                n.x += e.movementX / camera.zoom * moveSpeed;
                n.y += e.movementY / camera.zoom * moveSpeed;
                return n;
            });
        }
    }

    public onmouseup(e: MouseEvent) {
        this.mouseMoveHandlers = this.mouseMoveHandlers.filter((handler) => {
            return !handler.removeWhenMouseUp;
        });

        for(const handler of this.mouseUpHandlers) handler.handler(e);
        this.mouseUpHandlers = this.mouseUpHandlers.filter((handler) => {
            return !handler.once;
        });
    }

    private handleMultiSelectMousedown(event: MouseEvent, id: NodeID) {
        const selection = get(this.selection);

        // if this node isn't active, make it the active selection
        // and ensure it's part of the selection set
        // otherwise, remove it from the selection entirely
        if(selection.activeNode !== id) {
            selection.nodes.add(id);
            selection.activeNode = id;
        } else {
            selection.nodes.delete(id);
            if(selection.activeNode === id) {
                selection.activeNode = null;
            }
        }
        this.selection.set({ ...selection });
    }


    private onNodeMouseDown(e: MouseEvent, id: NodeID) {
        if(e.shiftKey) {
            this.handleMultiSelectMousedown(e, id);
            return;
        }

        this.editMode.set({ type: "drag-move", didMouseMove: false });
        
        const selection = get(this.selection);

        this.handleMouseUpOnce(() => {
            const editMode = get(this.editMode);
            if(editMode.type !== "drag-move") return;

            if(!editMode.didMouseMove) {
                // If the mouse didn't move, this was a click, so set ourself
                // as the active node
                selection.nodes.clear();
                selection.nodes.add(id);
                selection.activeNode = id;
                this.selection.set({ ...selection });
            }

            this.editMode.set({ type: "none" });
        });

        // If we're not part of the selection, set ourself as the sole active node
        if(!selection.nodes.has(id)) {
            selection.nodes.clear();
            selection.nodes.add(id);
            selection.activeNode = id;
            this.selection.set({ ...selection });
        }
    }

    onmousedown(e: MouseEvent) {
        switch(e.button) {
            case 0: // left
                const editMode = get(this.editMode);
                if(editMode.type !== "none") {
                    this.editMode.set({ type: "none" });
                    e.preventDefault();
                    e.stopPropagation();
                    return;
                }
        
                if(
                    document.activeElement instanceof HTMLElement &&
                    (document.activeElement.tagName === "INPUT" ||
                    document.activeElement.tagName === "TEXTAREA")
                ) {
                    document.activeElement.blur();
                    return;
                }
        
                if(!e.target || !(e.target instanceof HTMLElement)) return;
        
                const node = e.target.closest('[data-node-id]');
                if(node) {
                    this.onNodeMouseDown(e, node.getAttribute('data-node-id') as NodeID);
                    e.preventDefault();
                    e.stopPropagation();
                    return;
                }

                break;
            case 1: // middle
                break;
            case 2: // right
                break;
        }

        // Custom handlers (maybe not the right name?)
        for(const handler of this.mouseDownHandlers) handler.handler(e);
        this.mouseDownHandlers = this.mouseDownHandlers.filter((handler) => {
            return !handler.once;
        });
    }
}