use crate::enums::*;
use crate::handlers::command_handler::CommandHandler;
use crate::materials::material::Material;
use crate::materials::material_database::MaterialDatabase;
use crate::user_settings::UserSettingsDatabase;
use fraction::Fraction;
use serenity::{model::channel::Message, prelude::*};

pub struct RecipeCommandHandler;

impl CommandHandler for RecipeCommandHandler {
    fn handle_command(&self, context: &Context, message: &Message) {
        let tokens: Vec<&str> = message.content.split_ascii_whitespace().collect();
        let blank_vec: Vec<&str> = Vec::new();
        let (&command_name, command) = tokens.split_first().unwrap_or((&"", &blank_vec));
        if command_name.to_lowercase() == "!recipe" {
            let data = context.data.write();
            let user_settings_database = data.get::<UserSettingsDatabase>().unwrap();
            let material_database = data.get::<MaterialDatabase>().unwrap();
            let sent_message = message.channel_id.send_message(&context.http, |m| {
                let item_name = command.join(" ");
                let user_settings = user_settings_database.get(&message.author.name);
                let possible_total_raw =
                    material_database.lookup_result(&item_name, &user_settings);
                match possible_total_raw {
                    Some(total_raw) => m.embed(|e| {
                        let (amount_per_second, adjusted_total_raw) =
                            match user_settings.whole_numbers {
                                true => total_raw.whole_ratio(
                                    user_settings.ignore_furnaces_for_ratio,
                                    user_settings.ignore_raw_for_ratio,
                                ),
                                false => (Fraction::from(1), total_raw),
                            };
                        let mut producers: Vec<(&Material, (ProducerType, Fraction))> =
                            adjusted_total_raw.producers.into_iter().collect();
                        producers.sort_by(|(material_a, _), (material_b, _)| {
                            material_a.name.cmp(&material_b.name)
                        });
                        let mut total_raw: Vec<(&Material, Fraction)> =
                            adjusted_total_raw.total_raw.into_iter().collect();
                        total_raw.sort_by(|(material_a, _), (material_b, _)| {
                            material_a.name.cmp(&material_b.name)
                        });
                        let mut description_vec = producers
                            .into_iter()
                            .map(|(material, (producer_type, amount))| {
                                format!(
                                    "{}: **{}** {}{} required.",
                                    material.name,
                                    print_fraction(amount),
                                    get_producer_type_name(producer_type),
                                    if amount == Fraction::from(1) { "" } else { "s" }
                                )
                            })
                            .collect::<Vec<String>>();
                        description_vec.extend(vec!["".to_string()]);
                        description_vec.extend(
                            total_raw
                                .into_iter()
                                .map(|(material, amount)| {
                                    format!(
                                        "{}: **{}** required.",
                                        material.name,
                                        print_fraction(amount)
                                    )
                                })
                                .collect::<Vec<String>>(),
                        );
                        let description = description_vec.join("\n");
                        e.title(format!(
                            "Recipe for {} {} per second:",
                            amount_per_second, item_name
                        ))
                        .description(description)
                        .footer(|f| {
                            f.text(format!("Using Assembling Machine {} and {} Furnace", user_settings.assembling_machine_level, user_settings.furnace_level))
                        })
                    }),
                    None => m.content(format!("Unable to find item with name *{}*", item_name)),
                }
            });

            if let Err(why) = sent_message {
                println!("Unable to send message: {}", why);
            }
        }
    }
}

fn get_producer_type_name(producer_type: ProducerType) -> String {
    match producer_type {
        ProducerType::AssemblingMachine => "Assembling Machine".to_string(),
        ProducerType::ChemicalPlant => "Chemical Plant".to_string(),
        ProducerType::Furnace => "Furnace".to_string(),
        ProducerType::RocketSilo => "Rocket Silo".to_string(),
    }
}

fn print_fraction(fraction: Fraction) -> String {
    let fract = fraction.fract();
    if fract.numer().unwrap_or(&0) == &0 {
        fraction.to_string()
    } else if fraction.trunc() == Fraction::from(0) {
        fraction.to_string()
    } else {
        format!("{} {}", fraction.trunc(), fract)
    }
}
