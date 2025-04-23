import { tick } from "svelte";
import { Writable, writable } from "svelte/store";

export enum InterfaceTheme {
    Dark = "dark",
    Light = "light",
    System = "system"
};

function createSetting<SettingType extends string>(name: string, defaultValue: SettingType): Writable<SettingType> {
    const storageItemName = `setting_${name}`;
    const value = writable<SettingType>(localStorage.getItem(storageItemName) as SettingType ?? defaultValue);

    value.subscribe((newValue) => {
        localStorage.setItem(storageItemName, newValue);
    });

    return value;
}

export const theme = createSetting<InterfaceTheme>("theme", InterfaceTheme.Dark);

function getTheme(theme: InterfaceTheme): string {
    switch (theme) {
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
        document.documentElement.setAttribute("data-theme", getTheme(newValue));
        return;
    }

    await document.startViewTransition(async () => {
        await tick();
        document.documentElement.setAttribute("data-theme", getTheme(newValue));
    }).ready;

    const themeSelector = document.getElementById("themeSelector");
    if(themeSelector === null) {
        console.error("Theme selector not found");
        return;
    }

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
        duration: 1000,
        easing: 'ease-in-out',
        pseudoElement: '::view-transition-new(root)',
      }
    );
});