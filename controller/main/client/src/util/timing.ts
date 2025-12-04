/**
 * exponential decay function to smoothly interpolate between two values
 * while properly respecting delta time.
 * @param a
 * @param b
 * @param decay decay rate; reasonable values are around 1 to 10
 * @param dt
 */
export function expDecay(a: number, b: number, decay: number, dt: number) {
    return a + (b - a) * Math.exp(-decay * dt);
}

export function debounce(time: number): (fn: () => void) => void {
    let timeout: ReturnType<typeof setTimeout>;
    return (fn: () => void) => {
        if (timeout) {
            clearTimeout(timeout);
        }
        timeout = setTimeout(() => {
            fn();
        }, time);
    };
}