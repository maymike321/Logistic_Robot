use crate::materials::material::Material;
use crate::materials::materials_json_parser::parse_materials_json;
use crate::total_raw_result::TotalRawResult;
use crate::user_settings::UserSettings;
use serenity::prelude::TypeMapKey;

pub struct MaterialDatabase {
    materials: Vec<Material>,
}

impl MaterialDatabase {
    pub fn new(materials_json: String) -> Self {
        MaterialDatabase {
            materials: parse_materials_json(materials_json),
        }
    }
    pub fn lookup_result<'a>(
        &'a self,
        material_name: &str,
        user_settings: &'a UserSettings,
    ) -> Option<TotalRawResult<'a>> {
        match self
            .materials
            .iter()
            .find(|material| material.name.to_lowercase() == material_name.to_lowercase())
        {
            Some(material) => Some(material.total_raw_result(user_settings)),
            None => None,
        }
    }
}
impl TypeMapKey for MaterialDatabase {
    type Value = MaterialDatabase;
}
