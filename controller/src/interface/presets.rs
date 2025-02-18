use std::io::{Error, ErrorKind};

use serde::{Deserialize, Serialize};

use crate::render::effects::{AnyEffect, AnyTemporaryEffect};

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
        if let Ok(file) = std::fs::File::open(EFFECT_PRESET_FILE) {
            serde_json::from_reader(file).expect("Failed to load effect presets")
        } else {
            EffectPresets {
                presets: vec![],
                temporary_effects: vec![]
            }
        }
    }

    pub fn add_preset(&mut self, preset: EffectPreset) -> Result<(), Error> {
        // Ensure that the preset doesn't already exist
        if self.presets.iter().any(|existing_preset| existing_preset.name == preset.name) {
            return Err(Error::new(ErrorKind::AlreadyExists, "Preset already exists"));
        }

        self.presets.push(preset);
        self.save()?;
        Ok(())
    }

    pub fn add_temporary_effect(&mut self, preset: TemporaryEffectPreset) -> Result<(), Error> {
        // Ensure that the preset doesn't already exist
        if self.temporary_effects.iter().any(|existing_preset| existing_preset.name == preset.name) {
            return Err(Error::new(ErrorKind::AlreadyExists, "Effect already exists"));
        }

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

    pub fn get_preset_list(&self) -> Vec<shared::EffectPreset> {
        self.presets.iter().map(|preset| shared::EffectPreset {
            name: preset.name.clone(),
            icon: preset.icon.clone()
        }).collect()
    }

    fn save(&self) -> Result<(), Error> {
        let file = std::fs::File::create(EFFECT_PRESET_FILE)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}