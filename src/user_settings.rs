use crate::enums::FurnaceLevel;
use crate::enums::AssemblingMachineLevel;
use std::collections::HashMap;
use serenity::prelude::*;

#[derive(Clone, Debug)]
pub struct UserSettings {
    pub assembling_machine_level: AssemblingMachineLevel,
    pub furnace_level: FurnaceLevel,
    pub whole_numbers: bool,
    pub ignore_furnaces_for_ratio: bool,
    pub ignore_raw_for_ratio: bool
}
impl Default for UserSettings {
    fn default() -> Self {
        UserSettings { 
            assembling_machine_level: AssemblingMachineLevel::One, 
            furnace_level: FurnaceLevel::Stone,
            whole_numbers: false,
            ignore_furnaces_for_ratio: false, 
            ignore_raw_for_ratio: false 
        }
    }
}

pub struct UserSettingsDatabase {
    settings_hash_map: HashMap<String, UserSettings>
}
impl TypeMapKey for UserSettingsDatabase {
    type Value = UserSettingsDatabase;
}
impl UserSettingsDatabase {
    pub fn new() -> Self {
        UserSettingsDatabase { settings_hash_map: HashMap::new() }
    }

    pub fn update<F>(&mut self, name: &str, update_fun: F) where F: FnOnce(&mut UserSettings) {
        if let None = self.settings_hash_map.get(name) {
            self.settings_hash_map.insert(name.to_string(), UserSettings::default());
        }
        self.settings_hash_map
            .entry(name.to_string())
            .and_modify(update_fun);
    }

    pub fn get(&self, name: &str) -> UserSettings {
        if let Some(settings) = self.settings_hash_map.get(name) {
            return settings.clone();
        };
        UserSettings::default()
    }
}