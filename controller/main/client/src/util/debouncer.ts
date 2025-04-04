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