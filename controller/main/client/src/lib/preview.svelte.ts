import { writable } from "svelte/store";

export let previewedComponent = $state<string | null>(null);
export function setPreviewedComponent(component: string | null) {
    previewedComponent = component;
}