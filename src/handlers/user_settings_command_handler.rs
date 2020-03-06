use crate::handlers::command_handler::CommandHandler;
use crate::user_settings::UserSettingsDatabase;
use serenity::{model::channel::Message, prelude::*};

pub struct UserSettingsCommandHandler;

impl CommandHandler for UserSettingsCommandHandler {
    fn handle_command(&self, context: &Context, message: &Message) {
        let tokens: Vec<&str> = message.content.split_ascii_whitespace().collect();
        if tokens.first().unwrap_or(&"").to_lowercase() == "!user-settings" {
            let mut data = context.data.write();
            let user_settings_database = data.get_mut::<UserSettingsDatabase>().unwrap();
            let user_settings = user_settings_database.get(&message.author.name);
            let sent_message = message.channel_id.send_message(&context.http, |m| {
                m.content(format!("Settings for user {}:", &message.author.name))
                    .embed(|e| {
                        e.fields(vec![
                            (
                                "Assembly machine level:",
                                user_settings.assembly_machine_level.to_string(),
                                false,
                            ),
                            (
                                "Furnace level:",
                                user_settings.furnace_level.to_string(),
                                false,
                            ),
                            (
                                "Whole numbers in ratios:",
                                convert_bool_to_capital_string(user_settings.whole_numbers),
                                false,
                            ),
                            (
                                "Ignore furnaces for ratios:",
                                convert_bool_to_capital_string(
                                    user_settings.ignore_furnaces_for_ratio,
                                ),
                                false,
                            ),
                            (
                                "Ignore raw materials for ratios:",
                                convert_bool_to_capital_string(user_settings.ignore_raw_for_ratio),
                                false,
                            ),
                        ])
                    })
            });

            if let Err(why) = sent_message {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

fn convert_bool_to_capital_string(b: bool) -> String {
    match b {
        true => "True".to_string(),
        false => "False".to_string(),
    }
}
