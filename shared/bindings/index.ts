// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.

export type ClientToServerMessage = never;

export type EffectPreset = { name: string, icon: string, };

export type EffectPresetList = { effects: Array<EffectPreset>, };

export type InitializeMessage = { light_positions: Array<LightPosition>, effect_presets: Array<EffectPreset>, };

export type LightPosition = { x: number, y: number, };

export type MusicVisualizerMessage = { "type": "UpdateSpectrum" } & Array<number>;

export type ServerToClientMessage = { "type": "StatusUpdate" } & StatusUpdateMessage | { "type": "SystemStatusUpdate" } & SystemStatusUpdateMessage | { "type": "Initialize" } & InitializeMessage;

export type StatusUpdateMessage = { frames: number, average_window: number, average_frame_time: number, max_frame_time: number, min_frame_time: number, debug_text: string, };

export type SystemStatusUpdateMessage = { global_cpu: number, available_memory: number, total_memory: number, used_swap: number, };

export type TemporaryEffectList = { effects: Array<string>, };
