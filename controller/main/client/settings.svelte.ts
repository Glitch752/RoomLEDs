import { tick } from "svelte";
import { get, writable, type Writable } from "svelte/store";

export enum InterfaceTheme {
    Dark = "dark",
    Light = "light",
    System = "system"
};

function createSetting<SettingType = string>(name: string, defaultValue: SettingType): Writable<SettingType> {
    const storageItemName = `setting_${name}`;
    let initialValue;
    try {
        initialValue = localStorage.getItem(storageItemName) ? JSON.parse(localStorage.getItem(storageItemName)!) : defaultValue;
    } catch {
        initialValue = defaultValue;
    }

    const value = writable<SettingType>(initialValue);

    value.subscribe((newValue) => {
        localStorage.setItem(storageItemName, JSON.stringify(newValue));
    });

    return value;
}

export const theme = createSetting<InterfaceTheme>("theme", InterfaceTheme.Dark);

function getThemeId(theme: InterfaceTheme): string {
    switch(theme) {
        case InterfaceTheme.Dark:
            return "dark";
        case InterfaceTheme.Light:
            return "light";
        case InterfaceTheme.System:
            return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
    }
}

theme.subscribe(async (newValue) => {
    if(document.startViewTransition === undefined) {
        updateDocumentTheme();
        return;
    }

    await document.startViewTransition(async () => {
        await tick();
        updateDocumentTheme();
    });

    const themeSelector = document.getElementById("themeSelector");
    if(themeSelector === null) return;

    const rect = themeSelector.getBoundingClientRect();
    const x = rect.x + rect.width / 2;
    const y = rect.y + rect.height / 2;

    const left = x, top = y, right = window.innerWidth - x, bottom = window.innerHeight - y;
    const maxRadius = Math.hypot(Math.max(left, right), Math.max(top, bottom));

    document.documentElement.animate(
      {
        clipPath: [
            `circle(0px at ${x}px ${y}px)`,
            `circle(${maxRadius}px at ${x}px ${y}px)`
        ],
      },
      {
        duration: 200,
        easing: 'ease-in-out',
        pseudoElement: '::view-transition-new(root)',
      }
    );
});

export function updateDocumentTheme() {
    document.documentElement.setAttribute("data-theme", getThemeId(get(theme)));
}