export let previewedComponent = $state<{
    id: string | null
}>({
    id: null
});
export function setPreviewedComponent(component: string | null) {
    previewedComponent.id = component;
}