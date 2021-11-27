use serenity::framework::standard::CommandResult;
use serenity::framework::standard::macros::{command};
use crate::enums::{AssemblingMachineLevel, FurnaceLevel};
use crate::handlers::flags::Flags;
use crate::user_settings::{UserSettings, UserSettingsDatabase};
use serenity::{model::channel::Message, prelude::*};

#[command]
pub async fn update_settings(context: &Context, message: &Message) -> CommandResult {
    let command: Vec<&str> = message.content.split_ascii_whitespace().collect();
    let mut data = context.data.write().await;
    let user_settings_database = data.get_mut::<UserSettingsDatabase>().unwrap();
    let sent_message = message.channel_id.send_message(&context.http, |m| {
        let mut parsed_successfully = false;
        match Flags::new(command.to_vec()) {
            Ok(flags) => {
                for (flag_name, _possible_flag_value) in flags.get_all() {
                    match flag_name {
                        "a" | "f" | "w" | "ignore-furnace" | "ignore-raw" => {},
                        _ => {
                            m.content(format!("Error: Unknown flag *-{}*", flag_name));
                            return m;
                        }
                    }
                }
                user_settings_database.update(&message.author.name, |user_settings| {
                    let assembling_machine_level = get_assembling_machine_level(&user_settings, &flags);
                    if let Err(why) = assembling_machine_level { m.content(why); return; }

                    let furnace_level = get_furnace_level(&user_settings, &flags);
                    if let Err(why) = furnace_level { m.content(why); return; }

                    let whole_numbers = get_whole_number(&user_settings, &flags);
                    if let Err(why) = whole_numbers { m.content(why); return; }

                    let ignore_furnaces_for_ratio = get_ignore_furnace(&user_settings, &flags);
                    if let Err(why) = ignore_furnaces_for_ratio { m.content(why); return; }

                    let ignore_raw_for_ratio = get_ignore_raw(&user_settings, &flags);
                    if let Err(why) = ignore_raw_for_ratio { m.content(why); return; }

                    user_settings.assembling_machine_level = assembling_machine_level.unwrap();
                    user_settings.furnace_level = furnace_level.unwrap();
                    user_settings.whole_numbers = whole_numbers.unwrap();
                    user_settings.ignore_furnaces_for_ratio = ignore_furnaces_for_ratio.unwrap();
                    user_settings.ignore_raw_for_ratio = ignore_raw_for_ratio.unwrap();

                    parsed_successfully = true;
                });
            }
            Err(_) => {
                m.content("Invalid syntax.  Available flags are: -a, -f, -w, -ignore-furnace, -ignore-raw.  Type *-help update-settings* for help.");
                return m;
            }
        }
        if !parsed_successfully {
            return m;
        }
        m.content("User settings updated successfully.")
    });

    if let Err(why) = sent_message.await {
        println!("Unable to send message: {}", why);
    }
    Ok(())
}

fn get_assembling_machine_level(
    user_settings: &UserSettings,
    flags: &Flags,
) -> Result<AssemblingMachineLevel, String> {
    match flags.get("a") {
        Some(possible_flag_value) => match possible_flag_value {
            Some(flag_value) => match &*flag_value.to_lowercase() {
                "1" => Ok(AssemblingMachineLevel::One),
                "2" => Ok(AssemblingMachineLevel::Two),
                "3" => Ok(AssemblingMachineLevel::Three),
                _ => Err(format!(
                    "Error: Invalid assembling machine level *{}*",
                    flag_value
                )),
            },
            None => Err("No value found for -a flag".to_string()),
        },
        None => Ok(user_settings.assembling_machine_level),
    }
}

fn get_furnace_level(user_settings: &UserSettings, flags: &Flags) -> Result<FurnaceLevel, String> {
    match flags.get("f") {
        Some(possible_flag_value) => match possible_flag_value {
            Some(flag_value) => match &*flag_value.to_lowercase() {
                "stone" => Ok(FurnaceLevel::Stone),
                "steel" => Ok(FurnaceLevel::Steel),
                _ => Err(format!(
                    "Error: Invalid furnace level flag *{}*",
                    flag_value
                )),
            },
            None => Err("No value found for -f flag".to_string()),
        },
        None => Ok(user_settings.furnace_level),
    }
}

fn get_whole_number(user_settings: &UserSettings, flags: &Flags) -> Result<bool, String> {
    match flags.get("w") {
        Some(possible_flag_value) => match possible_flag_value {
            Some(flag_value) => match &*flag_value.to_lowercase() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(format!("Error: Invalid whole flag *{}*", flag_value)),
            },
            None => Err("No value found for -w flag".to_string()),
        },
        None => Ok(user_settings.whole_numbers),
    }
}

fn get_ignore_furnace(user_settings: &UserSettings, flags: &Flags) -> Result<bool, String> {
    match flags.get("ignore-furnace") {
        Some(possible_flag_value) => match possible_flag_value {
            Some(flag_value) => match &*flag_value.to_lowercase() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(format!("Invalid ignore furnace flag *{}*", flag_value)),
            },
            None => Err("Error: No value found for -ignore-furnace flag".to_string()),
        },
        None => Ok(user_settings.ignore_furnaces_for_ratio),
    }
}

fn get_ignore_raw(user_settings: &UserSettings, flags: &Flags) -> Result<bool, String> {
    match flags.get("ignore-raw") {
        Some(possible_flag_value) => match possible_flag_value {
            Some(flag_value) => match &*flag_value.to_lowercase() {
                "true" => Ok(true),
                "false" => Ok(false),
                _ => Err(format!("Invalid ignore raw flag *{}*", flag_value)),
            },
            None => Err("Error: No value found for -ignore-raw flag".to_string()),
        },
        None => Ok(user_settings.ignore_raw_for_ratio),
    }
}
