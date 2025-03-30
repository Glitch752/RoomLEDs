import type { AnyEffect, AnyTemporaryEffect } from "@bindings/index";
import { del, get, post } from ".";
import type { TemporaryEffectList, EffectPresetList } from "@shared-bindings/index";

export async function getTemporaryEffects(): Promise<TemporaryEffectList> {
    return await get<TemporaryEffectList>('/temporary_effects');
}

export async function deleteTemporaryEffect(id: string): Promise<void> {
    await del(`/temporary_effects/${id}`);
}

export async function createTemporaryEffect(id: string, effect: AnyTemporaryEffect): Promise<void> {
    return await post<void>(`/temporary_effects/${id}`, effect);
}

export async function getEffectPresets(): Promise<EffectPresetList> {
    return await get<EffectPresetList>('/effect_presets');
}

export async function getPresetData(id: string): Promise<AnyEffect> {
    return await get<AnyEffect>(`/effect_presets/${id}`);
}

export async function deleteEffectPreset(id: string): Promise<void> {
    await del(`/effect_presets/${id}`);
}

export async function createEffectPreset(id: string, fontAwesomeIcon: string, effect: AnyEffect): Promise<void> {
    return await post<void>(`/effect_presets/${id}?icon=${fontAwesomeIcon}`, effect);
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