use crate::enums::{AssemblyMachineLevel, FurnaceLevel, ProducerType};
use crate::recipe::Recipe;
use crate::total_raw_result::*;
use crate::user_settings::*;
use fraction::Fraction;
use std::collections::HashMap;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Material {
    pub name: String,
    pub recipe: Recipe,
}
impl Material {
    pub fn raw(name: &str) -> Material {
        Material {
            name: name.to_string(),
            recipe: Recipe::new(None, Vec::new(), Fraction::from(0), Fraction::from(1)),
        }
    }

    pub fn non_raw(name: &str, recipe: Recipe) -> Material {
        Material {
            name: name.to_string(),
            recipe: recipe,
        }
    }

    pub fn total_raw_result<'a>(&'a self, user_settings: &'a UserSettings) -> TotalRawResult<'a> {
        Material::total_raw_recurse(self, user_settings)
    }

    fn total_raw_recurse<'a>(
        item: &'a Material,
        user_settings: &'a UserSettings,
    ) -> TotalRawResult<'a> {
        let mut producers: HashMap<&Material, (ProducerType, Fraction)> = HashMap::new();
        let mut total_raw: HashMap<&Material, Fraction> = HashMap::new();
        if !item.is_raw() {
            let item_produced_factor = item.recipe.amount;
            for (recipe_item, recipe_amount) in &item.recipe.items {
                let product_result = Material::total_raw_recurse(&recipe_item, user_settings);

                let product_required_factor = *recipe_amount;
                let product_per_item = product_required_factor / item_produced_factor;

                Material::update_producers(
                    &mut producers,
                    product_result.producers,
                    product_per_item,
                );
                Material::update_raw(&mut total_raw, product_result.total_raw, product_per_item);
            }
            let production_modifier = Material::calculate_production_modifier(
                &user_settings.assembly_machine_level,
                &user_settings.furnace_level,
                &item.recipe.producer_type,
            );
            let time_per_item = item.recipe.time / production_modifier;

            producers.insert(
                item,
                (
                    item.recipe.producer_type.unwrap(),
                    time_per_item / item_produced_factor,
                ),
            );
        } else {
            total_raw.insert(item, Fraction::from(1));
        }

        TotalRawResult::new(producers, total_raw)
    }

    fn calculate_production_modifier(
        assembly_machine_level: &AssemblyMachineLevel,
        furnace_level: &FurnaceLevel,
        producer_type: &Option<ProducerType>,
    ) -> Fraction {
        match producer_type {
            Some(ProducerType::Furnace) => match furnace_level {
                FurnaceLevel::Stone => Fraction::from(1),
                FurnaceLevel::Steel => Fraction::from(2),
            },
            Some(ProducerType::AssemblyMachine) => match assembly_machine_level {
                AssemblyMachineLevel::One => Fraction::from(0.5),
                AssemblyMachineLevel::Two => Fraction::from(0.75),
                AssemblyMachineLevel::Three => Fraction::from(1.25),
            },
            _ => Fraction::from(1),
        }
    }

    fn update_producers<'a>(
        old_producers: &mut HashMap<&'a Material, (ProducerType, Fraction)>,
        new_producers: HashMap<&'a Material, (ProducerType, Fraction)>,
        product_per_item: Fraction,
    ) {
        for (material, (producer_type, producer_amount)) in new_producers {
            old_producers
                .entry(material)
                .and_modify(|(_producer_type, amount)| {
                    *amount = *amount + (producer_amount * product_per_item)
                })
                .or_insert((producer_type, producer_amount * product_per_item));
        }
    }

    fn update_raw<'a>(
        old_raw: &mut HashMap<&'a Material, Fraction>,
        new_raw: HashMap<&'a Material, Fraction>,
        product_per_item: Fraction,
    ) {
        for (material, amount_raw) in new_raw {
            old_raw
                .entry(material)
                .and_modify(|amount| *amount = *amount + (amount_raw * product_per_item))
                .or_insert(amount_raw * product_per_item);
        }
    }

    fn is_raw(&self) -> bool {
        self.recipe.items.len() == 0
    }
}
