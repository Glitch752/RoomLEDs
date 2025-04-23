use std::{io::{Error, ErrorKind}, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{render::{effects::{self, AnyEffect, AnyTemporaryEffect}, frame::PixelColor}, TOTAL_PIXELS};

static EFFECT_PRESET_FILE: &str = "effect_presets.json";

#[derive(Serialize, Deserialize)]
struct EffectPreset {
    name: String,
    icon: String,
    effect: AnyEffect
}

#[derive(Serialize, Deserialize)]
struct TemporaryEffectPreset {
    name: String,
    effect: AnyTemporaryEffect
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
                EffectPreset {
                    name: "Websocket Input".to_string(),
                    icon: "fas fa-plug".to_string(),
                    effect: effects::WebsocketInputEffect::new()
                },
                EffectPreset {
                    name: "Rainbow stripes".to_string(),
                    icon: "fas fa-rainbow".to_string(),
                    effect: effects::StripeEffect::new(TOTAL_PIXELS as f64 / 28., vec![
                        (255, 0, 0).into(),
                        (255, 100, 0).into(),
                        (255, 255, 0).into(),
                        (0, 255, 0).into(),
                        (0, 0, 255).into(),
                        (143, 0, 255).into(),
                        (255, 255, 255).into(),
                    ], 84.0)
                },
                EffectPreset {
                    name: "Music visualizer".to_string(),
                    icon: "fas fa-music".to_string(),
                    effect: effects::RotateEffect::new(
                        effects::MusicVisualizerEffect::new(shared::constants::MUSIC_VISUALIZER_PORT).into(),
                        -219
                    )
                },
                EffectPreset {
                    name: "Flashing red".to_string(),
                    icon: "fas fa-bolt".to_string(),
                    effect: effects::FlashingColorEffect::new(1., 0., PixelColor::new(255, 0, 0, 1.0), PixelColor::new(255, 0, 0, 0.0)).into()
                },
                EffectPreset {
                    name: "Solid white".to_string(),
                    icon: "fas fa-sun".to_string(),
                    effect: effects::SolidColorEffect::new(PixelColor::new(255, 255, 255, 1.0), 0, TOTAL_PIXELS)
                },
                EffectPreset {
                    name: "Solid black".to_string(),
                    icon: "fas fa-moon".to_string(),
                    effect: effects::SolidColorEffect::new(PixelColor::new(0, 0, 0, 1.0), 0, TOTAL_PIXELS)
                },
            ],
            temporary_effects: vec![]
        }
    }

    pub fn add_preset(&mut self, name: String, icon: String, effect: AnyEffect) -> Result<(), Error> {
        // If the preset already exists, update it
        if let Some(existing_preset) = self.presets.iter_mut().find(|existing_preset| existing_preset.name == name) {
            existing_preset.icon = icon;
            existing_preset.effect = effect;
            self.save()?;
            return Ok(());
        }

        let preset = EffectPreset {
            name,
            icon,
            effect
        };
        self.presets.push(preset);
        self.save()?;
        Ok(())
    }

    pub fn add_temporary_effect(&mut self, name: String, effect: AnyTemporaryEffect) -> Result<(), Error> {
        // If the effect already exists, update it
        if let Some(existing_preset) = self.temporary_effects.iter_mut().find(|existing_preset| existing_preset.name == name) {
            existing_preset.effect = effect;
            self.save()?;
            return Ok(());
        }

        let preset = TemporaryEffectPreset {
            name,
            effect
        };
        self.temporary_effects.push(preset);
        self.save()?;
        Ok(())
    }

    pub fn remove_preset(&mut self, name: &str) -> Result<(), Error> {
        let index = self.presets.iter().position(|preset| preset.name == name);
        if let Some(index) = index {
            self.presets.remove(index);
            self.save()?;
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NotFound, "Preset not found"))
        }
    }

    pub fn remove_temporary_effect(&mut self, name: &str) -> Result<(), Error> {
        let index = self.temporary_effects.iter().position(|preset| preset.name == name);
        if let Some(index) = index {
            self.temporary_effects.remove(index);
            self.save()?;
            Ok(())
        } else {
            Err(Error::new(ErrorKind::NotFound, "Effect not found"))
        }
    }

    pub fn get_preset(&self, name: &str) -> Option<AnyEffect> {
        self.presets.iter().find(|preset| preset.name == name).map(|preset| preset.effect.clone())
    }

    pub fn get_temporary_effect(&self, name: &str) -> Option<AnyTemporaryEffect> {
        self.temporary_effects.iter().find(|preset| preset.name == name).map(|preset| preset.effect.clone())
    }

    pub fn get_temporary_effect_list(&self) -> Vec<String> {
        self.temporary_effects.iter().map(|preset| preset.name.clone()).collect()
    }

    pub fn get_preset_list(&self) -> Vec<shared::EffectPreset> {
        self.presets.iter().map(|preset| shared::EffectPreset {
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