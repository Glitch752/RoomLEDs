import type { AnyEffect, AnyTemporaryEffect } from "@bindings/index";
import { del, get, post, put } from ".";
import type { TemporaryEffectList, EffectPresetList } from "@shared-bindings/index";

export async function getTemporaryEffects(): Promise<TemporaryEffectList> {
    return await get<TemporaryEffectList>('/temporary_effects');
}

export async function deleteTemporaryEffect(id: string): Promise<void> {
    await del(`/temporary_effect/${id}`);
}

export async function createTemporaryEffect(name: string, effect: AnyTemporaryEffect): Promise<void> {
    return await post<void>(`/temporary_effect?name=${name}`, effect);
}

export async function updateTemporaryEffect(id: string, newName: string, newEffect: AnyTemporaryEffect): Promise<void> {
    return await put<void>(`/temporary_effect/${id}?name=${newName}`, newEffect);
}

export async function getEffectPresets(): Promise<EffectPresetList> {
    return await get<EffectPresetList>('/effect_presets');
}

export async function getPresetData(id: string): Promise<AnyEffect> {
    return await get<AnyEffect>(`/effect_preset/${id}`);
}

export async function deleteEffectPreset(id: string): Promise<void> {
    await del(`/effect_preset/${id}`);
}

export async function createEffectPreset(name: string, fontAwesomeIcon: string, effect: AnyEffect): Promise<void> {
    return await post<void>(`/effect_preset?name=${name}&icon=${fontAwesomeIcon}`, effect);
}

export async function updateEffectPreset(id: string, newName: string, newIcon: string, newEffect: AnyEffect): Promise<void> {
    return await put<void>(`/effect_preset/${id}?name=${newName}&icon=${newIcon}`, newEffect);
}

export async function runArbitraryEffect(effect: AnyEffect | null): Promise<void> {
    return await post<void>('/run_effect', effect);
}

export async function runEffectPreset(id: string): Promise<void> {
    return await post<void>(`/run_effect/${id}`);
}

export async function runTemporaryEffect(id: string): Promise<void> {
    return await post<void>(`/run_temporary_effect/${id}`);
}