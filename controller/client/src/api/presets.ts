import { del, get, post } from ".";
import type { TemporaryEffectList } from "@shared-bindings/TemporaryEffectList";
import type { AnyTemporaryEffect } from "@bindings/AnyTemporaryEffect";
import type { EffectPresetList } from "@shared-bindings/EffectPresetList";
import type { AnyEffect } from "@bindings/AnyEffect";

export async function getTemporaryEffects(): Promise<TemporaryEffectList> {
    return await get<TemporaryEffectList>('/temporary-effects');
}

export async function deleteTemporaryEffect(id: string): Promise<void> {
    await del(`/temporary-effects/${id}`);
}

export async function createTemporaryEffect(id: string, effect: AnyTemporaryEffect): Promise<void> {
    return await post<void>(`/temporary-effects/${id}`, effect);
}

export async function getEffectPresets(): Promise<EffectPresetList> {
    return await get<EffectPresetList>('/effect-presets');
}

export async function deleteEffectPreset(id: string): Promise<void> {
    await del(`/effect-presets/${id}`);
}

export async function createEffectPreset(id: string, fontAwesomeIcon: string, effect: AnyEffect): Promise<void> {
    return await post<void>(`/effect-presets/${id}?icon=${fontAwesomeIcon}`, effect);
}

export async function runArbitraryEffect(effect: AnyEffect): Promise<void> {
    return await post<void>('/run_effect', effect);
}

export async function runEffectPreset(id: string): Promise<void> {
    return await post<void>(`/run_effect/${id}`);
}

export async function runTemporaryEffect(id: string): Promise<void> {
    return await post<void>(`/run_temporary_effect/${id}`);
}