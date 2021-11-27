use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::{command};
use crate::enums::ProducerType;
use crate::materials::material::Material;
use crate::materials::material_database::MaterialDatabase;
use crate::user_settings::UserSettingsDatabase;
use fraction::Fraction;
use serenity::{model::channel::Message, prelude::Context};

#[command]
pub async fn recipe(context: &Context, message: &Message) -> CommandResult {
    let tokens: Vec<&str> = message.content.split_ascii_whitespace().collect();
    let mut amount = 1;
    let command;
    if tokens[tokens.len() - 2] == "-a" {
        let split = tokens.split_at(tokens.len() - 2);
        command = split.0.to_vec();
        amount = split.1[1].parse::<i32>().unwrap_or(1);
    }
    else {
        command = tokens;
    }
    let data = context.data.write().await;
    let user_settings_database = data.get::<UserSettingsDatabase>().unwrap();
    let material_database = data.get::<MaterialDatabase>().unwrap();
    let item_name = command.join(" ");
    let user_settings = user_settings_database.get(&message.author.name);
    let possible_total_raw =
        material_database.lookup_result(&item_name, &user_settings);
    let sent_message = message.channel_id.send_message(&context.http, |m| {
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
                e.title(format!(
                    "Recipe for {} {} per second:",
                    amount, item_name
                ))
                .description(generate_description(producers, total_raw, Fraction::from(amount) / amount_per_second))
                .footer(|f| {
                    f.text(format!(
                        "Using Assembling Machine {} and {} Furnace",
                        user_settings.assembling_machine_level, user_settings.furnace_level
                    ))
                })
            }),
            None => m.content(format!("Unable to find item with name *{}*", item_name)),
        }
    });

    if let Err(why) = sent_message.await {
        println!("Unable to send message: {}", why);
    }

    Ok(())
}

fn generate_description(
    producers: Vec<(&Material, (ProducerType, Fraction))>,
    total_raw: Vec<(&Material, Fraction)>,
    ratio: Fraction
) -> String {
    let mut description_vec = producers
        .into_iter()
        .map(|(material, (producer_type, amount))| {
            format!(
                "{}: **{}** {}{} required.",
                material.name,
                print_fraction(amount * ratio),
                get_producer_type_name(producer_type),
                if amount * ratio == Fraction::from(1) { "" } else { "s" }
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
                    print_fraction(amount * ratio)
                )
            })
            .collect::<Vec<String>>(),
    );
    description_vec.join("\n")
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
