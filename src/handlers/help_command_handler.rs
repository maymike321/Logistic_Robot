use crate::handlers::command_handler::CommandHandler;
use serenity::{
    builder::{CreateEmbed, CreateMessage},
    model::channel::Message,
    prelude::*,
};

pub struct HelpCommandHandler;

impl CommandHandler for HelpCommandHandler {
    fn handle_command(&self, context: &Context, message: &Message) {
        let tokens: Vec<&str> = message.content.split_ascii_whitespace().collect();
        if tokens.first().unwrap_or(&"").to_lowercase() == "!help" {
            let sent_message = match tokens.get(1) {
                None => message
                    .channel_id
                    .send_message(&context.http, create_help_message),
                Some(&"user-settings") => message
                    .channel_id
                    .send_message(&context.http, create_settings_help_message),
                Some(&"update-settings") => message
                    .channel_id
                    .send_message(&context.http, create_update_settings_help_message),
                Some(_) => message
                    .channel_id
                    .send_message(&context.http, create_help_message),
            };

            if let Err(why) = sent_message {
                println!("Error sending message: {:?}", why);
            }
        }
    }
}

fn create_help_message<'a, 'b>(message: &'a mut CreateMessage<'b>) -> &'a mut CreateMessage<'b> {
    message
        .embed(|e| {
            e.title("List of available commands:").fields(vec![
                ("!recipe", "Gives the exact amount of machines needed to create an item.\nExample: *!recipe logistic science pack*\nAdd -a x to calculate for x items a second.\nExample: *!recipe logistic science pack -a 5*", false),
                ("!user-settings", "Displays your current user settings, which modify !recipe calculations.\nType *!help user-settings* for more information.", false),
                ("!update-settings", "Updates user settings.\nType *!help update-settings* for more information.", false)
            ])
    })
}

fn create_settings_help_message<'a, 'b>(
    message: &'a mut CreateMessage<'b>,
) -> &'a mut CreateMessage<'b> {
    message.embed(|e| {
        e.title("User settings:");
        add_user_settings_flags(e)
    })
}

fn create_update_settings_help_message<'a, 'b>(
    message: &'a mut CreateMessage<'b>,
) -> &'a mut CreateMessage<'b> {
    message
        .embed(|e| {
            e.title("Update settings:")
                .description("Updates user settings which are used to calculate ratios.\n\nExample: *!update-settings -a 3 -f stone*\n\nAvailable flags:");
            add_user_settings_flags(e)
        })
}

fn add_user_settings_flags(embed: &mut CreateEmbed) -> &mut CreateEmbed {
    embed.fields(vec![
        ("-a", "Assembling machine level.\nValues: *1, 2, 3*", false),
        ("-f", "Furnace level.\nValues: *Stone, Steel*", false),
        (
            "-w",
            "Adjust ratios to all be whole numbers.\nValues: *true, false*",
            false,
        ),
        (
            "-ignore-furnace",
            "Ignore furnaces when adjusting ratios to whole numbers.\nValues: *true, false*",
            false,
        ),
        (
            "-ignore-raw",
            "Ignore raw materials when adjusting ratios to whole numbers.\nValues: *true, false*",
            false,
        ),
    ])
}
