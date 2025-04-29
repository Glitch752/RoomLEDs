use std::{io::{Error, ErrorKind}, path::PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{render::{effects::{self, AnyEffect, AnyTemporaryEffect}, frame::PixelColor}, TOTAL_PIXELS};

static EFFECT_PRESET_FILE: &str = "effect_presets.json";

#[derive(Serialize, Deserialize)]
struct EffectPreset {
    name: String,
    icon: String,
    effect: AnyEffect,
    id: Uuid
}

impl EffectPreset {
    fn new(name: String, icon: String, effect: AnyEffect) -> Self {
        EffectPreset {
            name,
            icon,
            effect,
            id: Uuid::new_v4()
        }
    }
}

#[derive(Serialize, Deserialize)]
struct TemporaryEffectPreset {
    name: String,
    effect: AnyTemporaryEffect,
    id: Uuid
}

impl TemporaryEffectPreset {
    fn new(name: String, effect: AnyTemporaryEffect) -> Self {
        TemporaryEffectPreset {
            name,
            effect,
            id: Uuid::new_v4()
        }
    }
}

/// Stores the web interface effect presets and persists them to disk.
#[derive(Serialize, Deserialize)]
pub(crate) struct EffectPresets {
    presets: Vec<EffectPreset>,
    temporary_effects: Vec<TemporaryEffectPreset>
}

impl EffectPresets {
    pub fn load() -> Self {
        if let Ok(file) = std::fs::File::open(EffectPresets::get_file_path()) {
            // TODO: More robust handling of schema changes
            match serde_json::from_reader(file) {
                Ok(presets) => {
                    println!("Loaded effect presets from file");
                    return presets;
                }
                Err(e) => {
                    println!("Failed to load effect presets from file: {}; reverting to default list", e);
                }
            }
        } else {
            println!("No effect presets found; starting with a default list");
        }

        EffectPresets {
            presets: vec![
                EffectPreset::new(
                    "Websocket Input".to_string(),
                    "fas fa-plug".to_string(),
                    effects::WebsocketInputEffect::new()
                ),
                EffectPreset::new(
                    "Rainbow stripes".to_string(),
                    "fas fa-rainbow".to_string(),
                    effects::StripeEffect::new(TOTAL_PIXELS as f64 / 28., vec![
                        (255, 0, 0).into(),
                        (255, 100, 0).into(),
                        (255, 255, 0).into(),
                        (0, 255, 0).into(),
                        (0, 0, 255).into(),
                        (143, 0, 255).into(),
                        (255, 255, 255).into(),
                    ], 84.0)
                ),
                EffectPreset::new(
                    "Music visualizer".to_string(),
                    "fas fa-music".to_string(),
                    effects::RotateEffect::new(
                        effects::MusicVisualizerEffect::new(shared::constants::MUSIC_VISUALIZER_PORT).into(),
                        -219
                    )
                ),
                EffectPreset::new(
                    "Flashing red".to_string(),
                    "fas fa-bolt".to_string(),
                    effects::FlashingColorEffect::new(1., 0., PixelColor::new(255, 0, 0, 1.0), PixelColor::new(255, 0, 0, 0.0)).into()
                ),
                EffectPreset::new(
                    "Solid white".to_string(),
                    "fas fa-sun".to_string(),
                    effects::SolidColorEffect::new(PixelColor::new(255, 255, 255, 1.0), 0, TOTAL_PIXELS)
                ),
                EffectPreset::new(
                    "Solid black".to_string(),
                    "fas fa-moon".to_string(),
                    effects::SolidColorEffect::new(PixelColor::new(0, 0, 0, 1.0), 0, TOTAL_PIXELS)
                )
            ],
            temporary_effects: vec![]
        }
    }

    pub fn add_preset(&mut self, name: String, icon: String, effect: AnyEffect) -> Result<(), Error> {
        let preset = EffectPreset::new(
            name,
            icon,
            effect
        );
        self.presets.push(preset);
        self.save()?;
        Ok(())
    }

    pub fn add_temporary_effect(&mut self, name: String, effect: AnyTemporaryEffect) -> Result<(), Error> {
        let preset = TemporaryEffectPreset::new(name, effect);
        self.temporary_effects.push(preset);
        self.save()?;
        Ok(())
    }

    pub fn update_preset(&mut self, id: Uuid, name: String, icon: String, effect: AnyEffect) -> Result<(), Error> {
        let index = self.presets.iter().position(|preset| preset.id == id);
        if let Some(index) = index {
            self.presets[index] = EffectPreset {
                name,
                icon,
                effect,
                id
            };
            self.save()?;
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NotFound, "Preset not found"))
        }
    }

    pub fn update_temporary_effect(&mut self, id: Uuid, name: String, effect: AnyTemporaryEffect) -> Result<(), Error> {
        let index = self.temporary_effects.iter().position(|preset| preset.id == id);
        if let Some(index) = index {
            self.temporary_effects[index] = TemporaryEffectPreset {
                name,
                effect,
                id
            };
            self.save()?;
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NotFound, "Effect not found"))
        }
    }

    pub fn remove_preset(&mut self, id: Uuid) -> Result<(), Error> {
        let index = self.presets.iter().position(|preset| preset.id == id);
        if let Some(index) = index {
            self.presets.remove(index);
            self.save()?;
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NotFound, "Preset not found"))
        }
    }

    pub fn remove_temporary_effect(&mut self, id: Uuid) -> Result<(), Error> {
        let index = self.temporary_effects.iter().position(|preset| preset.id == id);
        if let Some(index) = index {
            self.temporary_effects.remove(index);
            self.save()?;
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NotFound, "Effect not found"))
        }
    }

    pub fn get_preset(&self, uuid: Uuid) -> Option<AnyEffect> {
        self.presets.iter().find(|preset| preset.id == uuid).map(|preset| preset.effect.clone())
    }

    pub fn get_temporary_effect(&self, uuid: Uuid) -> Option<AnyTemporaryEffect> {
        self.temporary_effects.iter().find(|preset| preset.id == uuid).map(|preset| preset.effect.clone())
    }

    pub fn get_temporary_effect_list(&self) -> Vec<shared::TemporaryEffect> {
        self.temporary_effects.iter().map(|preset| shared::TemporaryEffect {
            id: preset.id.to_string(),
            name: preset.name.clone()
        }).collect()
    }

    pub fn get_preset_list(&self) -> Vec<shared::EffectPreset> {
        self.presets.iter().map(|preset| shared::EffectPreset {
            id: preset.id.to_string(),
            name: preset.name.clone(),
            icon: preset.icon.clone()
        }).collect()
    }

    fn save(&self) -> Result<(), Error> {
        let file = std::fs::File::create(EffectPresets::get_file_path())?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }

    fn get_file_path() -> PathBuf {
        dirs::data_dir().unwrap().join(EFFECT_PRESET_FILE)
    }
}