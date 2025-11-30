export default class CallbackContainer {
    private callbacks: Set<() => void> = new Set();
    
    addCallback(callback: () => void) {
        this.callbacks.add(callback);
    }
    
    removeCallback(callback: () => void) {
        this.callbacks.delete(callback);
    }

    invokeAll() {
        this.callbacks.forEach(callback => callback());
    }
}