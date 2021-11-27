extern crate serenity;

use crate::handlers::help_command_handler::HELP_COMMAND;
use crate::handlers::recipe_command_handler::RECIPE_COMMAND;
use crate::handlers::update_settings_command_handler::UPDATE_SETTINGS_COMMAND;
use crate::handlers::user_settings_command_handler::USER_SETTINGS_COMMAND;
use crate::user_settings::UserSettingsDatabase;
use serenity::framework::standard::macros::{group};
use serenity::framework::standard::{StandardFramework};

mod enums;
mod handlers;
mod materials;
mod recipe;
mod total_raw_result;
mod user_settings;

#[group]
#[commands(recipe, user_settings, update_settings, help)]
struct General;

#[tokio::main]
async fn main() {
    use materials::material_database::MaterialDatabase;
    let framework = StandardFramework::new()
        .configure(|c| {
            c.prefix("!")
        })
        .group(&GENERAL_GROUP);
    let token = std::env::var("TOKEN").expect("TOKEN environment variable not set.");
    let mut client = serenity::Client::builder(token)
        .framework(framework)
        .await
        .expect("Error creating client");
    {
        let mut data = client.data.write().await;

        let file_name = "src/materials/materials.json";
        let json_file_contents = std::fs::read_to_string(file_name)
            .expect(&format!("Unable to find file {}", file_name));

        data.insert::<UserSettingsDatabase>(UserSettingsDatabase::new());
        data.insert::<MaterialDatabase>(MaterialDatabase::new(json_file_contents));
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
