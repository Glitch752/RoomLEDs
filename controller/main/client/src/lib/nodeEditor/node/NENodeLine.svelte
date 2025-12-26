<script lang="ts">
    import type NEDocumentState from "../NEDocumentState";
    import type { EdgeData, NodeData, NodeID, EdgeID } from "../NodeTypes";
    import { nodeDataTypeInfo, type NodeDataType } from "../NodeVariants";

    const {
        isInput,
        text,
        type,
        nodeState,
        node,
        index
    }: {
        isInput: boolean,
        text: string,
        type: NodeDataType,
        nodeState: NEDocumentState,
        node: NodeData,
        index: number
    } = $props();

    const typeInfo = nodeDataTypeInfo[type];

    function onmousemove(event: MouseEvent) {
        // TODO: do the thing where the camera moves when dragging near the edge of the
        // viewport

        nodeState.draggingEdge.update(edge => {
            if(!edge) return null;

            if(edge.type === "from") {
                // if hovering over an input socket that isn't on the source node, snap to it
                let to = {
                    type: 'point',
                    ...nodeState.mouseEventToCanvasPos(event)
                } as typeof edge.to;

                const element = document.elementFromPoint(event.clientX, event.clientY);
                if(element && element.classList.contains("socket")) {
                    const inputLineElement = element.closest(".line");
                    if(inputLineElement) {
                        const inputNodeId = inputLineElement.getAttribute("data-node-id");
                        const inputIndexAttr = inputLineElement.getAttribute("data-input-index");
                        if(inputNodeId && inputIndexAttr) {
                            const inputIndex = parseInt(inputIndexAttr);
                            if(inputNodeId !== edge.fromNodeId) {
                                to = {
                                    type: 'nodeInput',
                                    nodeId: inputNodeId as NodeID,
                                    inputIndex
                                };
                            }
                        }
                    }
                }

                return {
                    ...edge,
                    to
                } satisfies typeof edge;
            } else {
                // same for outputs
                let from = {
                    type: 'point',
                    ...nodeState.mouseEventToCanvasPos(event)
                } as typeof edge.from;

                const element = document.elementFromPoint(event.clientX, event.clientY);
                if(element && element.classList.contains("socket")) {
                    const outputLineElement = element.closest(".line");
                    if(outputLineElement) {
                        const outputNodeId = outputLineElement.getAttribute("data-node-id");
                        const outputIndexAttr = outputLineElement.getAttribute("data-output-index");
                        if(outputNodeId && outputIndexAttr) {
                            const outputIndex = parseInt(outputIndexAttr);
                            if(outputNodeId !== edge.toNodeId) {
                                from = {
                                    type: 'nodeOutput',
                                    nodeId: outputNodeId as NodeID,
                                    outputIndex
                                };
                            }
                        }
                    }
                }

                return {
                    ...edge,
                    from
                } satisfies typeof edge;
            }
        });
    }

    function onmouseup(event: MouseEvent) {
        window.removeEventListener("mousemove", onmousemove);

        nodeState.draggingEdge.update((edge) => {
            // If both ends are connected to nodes, create a new edge
            if(!edge) return null;

            let newEdge: EdgeData = {
                id: crypto.randomUUID() as EdgeID,
                from: {
                    nodeId: "" as NodeID,
                    outputIndex: 0
                },
                to: {
                    nodeId: "" as NodeID,
                    inputIndex: 0
                }
            };

            if(edge.type === "from") {
                if(edge.to.type === "point") return null;

                newEdge.from.nodeId = edge.fromNodeId;
                newEdge.from.outputIndex = edge.fromOutputIndex;
                newEdge.to.nodeId = edge.to.nodeId;
                newEdge.to.inputIndex = edge.to.inputIndex;
            } else {
                if(edge.from.type === "point") return null;

                newEdge.from.nodeId = edge.from.nodeId;
                newEdge.from.outputIndex = edge.from.outputIndex;
                newEdge.to.nodeId = edge.toNodeId;
                newEdge.to.inputIndex = edge.toInputIndex;
            }

            // if there's already a edge connected to this input, remove it
            const existingEdge = nodeState.getInputEdge(newEdge.to.nodeId, newEdge.to.inputIndex);
            if(existingEdge) {
                nodeState.removeEdge(existingEdge);
            }

            nodeState.addEdge(newEdge);
            
            return null;
        });
    }

    function onmousedown(event: MouseEvent) {
        event.preventDefault();
        event.stopPropagation();

        if(isInput) {
            // if the input already has a connection, remove it
            // and begin re-dragging the old connection.
            // otherwise, start dragging a new one
            const inputEdge = nodeState.getInputEdge(node.id, index);
            if(inputEdge) {
                const inputEdgeValue = nodeState.getEdgeValue(inputEdge);
                nodeState.removeEdge(inputEdge);
                nodeState.draggingEdge.set({
                    type: "from",
                    fromNodeId: inputEdgeValue.from.nodeId,
                    fromOutputIndex: inputEdgeValue.from.outputIndex,
                    to: {
                        type: 'point',
                        ...nodeState.mouseEventToCanvasPos(event)
                    }
                });
            } else {
                nodeState.draggingEdge.set({
                    type: "to",
                    toNodeId: node.id,
                    toInputIndex: index,
                    from: {
                        type: 'point',
                        ...nodeState.mouseEventToCanvasPos(event)
                    }
                })
            }
        } else {
            // this is an output; always start dragging a new edge.
            nodeState.draggingEdge.set({
                type: "from",
                fromNodeId: node.id,
                fromOutputIndex: index,
                to: {
                    type: 'point',
                    ...nodeState.mouseEventToCanvasPos(event)
                }
            });
        }

        window.addEventListener("mouseup", onmouseup, { once: true });
        window.addEventListener("mousemove", onmousemove);
    }
</script>

<!-- svelte-ignore a11y_no_static_element_interactions -->
<div
    class="line"
    class:input={isInput}
    class:output={!isInput}
    
    data-node-id={node.id}
    data-input-index={isInput ? index : undefined}
    data-output-index={!isInput ? index : undefined}

    title={`${text}
    Type: ${typeInfo.label}`}
>
    <span class="content">{text}</span>
    <div
        class="socket"
        style="--color: {typeInfo.primaryColor}; --lightColor: {typeInfo.lightColor};"
        {onmousedown}
    ></div>
</div>

<style lang="scss">
.line {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 0.75rem;
    gap: 0.7rem;
    position: relative;

    .content {
        font-size: 0.85rem;
        flex: 1;
    }
    &.output .content {
        text-align: right;
    }
    &.input .content {
        text-align: left;
    }

    .socket {
        --selection-width: 1rem;

        cursor: pointer;
        height: 100%;
        width: var(--selection-width);
        position: absolute;
        display: flex;
        align-items: center;
        justify-content: center;
    }

    // use a pseudoelement because we want a large area around the socket to be selectable
    .socket::before {
        content: '';
        display: block;
        width: 6px;
        height: 10px;
        background: var(--color, var(--subtext0));
        border: 1px solid var(--lightColor, var(--subtext1));
        border-radius: 3px;
    }
    &.input .socket {
        left: calc(var(--selection-width) * -0.5);
    }
    &.output .socket {
        right: calc(var(--selection-width) * -0.5);
    }
}
</style>