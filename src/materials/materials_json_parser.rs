use crate::enums::ProducerType;
use crate::materials::material::Material;
use crate::recipe::*;
use fraction::Fraction;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct JsonFileResult {
    pub raw: Vec<String>,
    pub non_raw: Vec<NonRawMaterialContract>,
}

#[derive(Clone, Serialize, Deserialize)]
struct NonRawMaterialContract {
    pub name: String,
    pub producer: String,
    pub resources: Vec<Resource>,
    pub amount: f32,
    pub time: f32,
}

#[derive(Serialize, Deserialize, Clone)]
struct Resource {
    pub name: String,
    pub amount: f32,
}

pub fn parse_materials_json(json_file_contents: String) -> Vec<Material> {
    let parsed_json: JsonFileResult = serde_json::from_str(&json_file_contents)
        .expect(&format!("Error parsing json file contents"));
    let raw_materials: Vec<Material> = parsed_json
        .raw
        .into_iter()
        .map(|material_contract| Material::raw(&material_contract.clone()))
        .collect();
    let unparsed_materials = parsed_json.non_raw.to_vec();
    let mut parsed_materials = raw_materials.to_vec();
    parsed_materials.extend(parsed_json.non_raw.into_iter().map(|unparsed_material| {
        parse_material(unparsed_material, &raw_materials, &unparsed_materials)
    }));
    parsed_materials
}

fn parse_material(
    unparsed_material: NonRawMaterialContract,
    raw_materials: &Vec<Material>,
    unparsed_materials: &Vec<NonRawMaterialContract>,
) -> Material {
    let mut recipe_materials: Vec<(Box<Material>, Fraction)> = Vec::new();
    for resource in unparsed_material.resources.clone() {
        let resource_material: Box<Material>;
        if let Some(material) = raw_materials
            .iter()
            .find(|material| material.name == resource.name)
        {
            resource_material = Box::new(Material::raw(&material.name.clone()));
        } else if let Some(material) = unparsed_materials
            .iter()
            .find(|material| material.name == resource.name)
        {
            resource_material = Box::new(parse_material(
                material.clone(),
                raw_materials,
                unparsed_materials,
            ));
        } else {
            panic!("Unable to find recipe name: {}", resource.name)
        }
        recipe_materials.push((resource_material, Fraction::from(resource.amount)));
    }
    let recipe: Recipe = Recipe::new(
        Some(get_producer_type(&unparsed_material.producer)),
        recipe_materials,
        Fraction::from(unparsed_material.time),
        Fraction::from(unparsed_material.amount),
    );
    Material::non_raw(&unparsed_material.name.clone(), recipe)
}

fn get_producer_type(s: &str) -> ProducerType {
    match s {
        "Assembling Machine" => ProducerType::AssemblingMachine,
        "Furnace" => ProducerType::Furnace,
        "Chemical Plant" => ProducerType::ChemicalPlant,
        "Rocket Silo" => ProducerType::RocketSilo,
        _ => panic!("Invalid producer type {}", s),
    }
}
