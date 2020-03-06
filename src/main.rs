extern crate serenity;

use crate::handlers::recipe_command_handler::RecipeCommandHandler;
use crate::handlers::handler::Handler;
use crate::handlers::help_command_handler::HelpCommandHandler;
use crate::handlers::user_settings_command_handler::UserSettingsCommandHandler;
use crate::handlers::update_settings_command_handler::UpdateSettingsCommandHandler;
use crate::user_settings::UserSettingsDatabase;

mod enums;
mod handlers;
mod materials;
mod recipe;
mod total_raw_result;
mod user_settings;

fn main() {
    use materials::material_database::MaterialDatabase;
    let update_settings_command_handler: UpdateSettingsCommandHandler =
        UpdateSettingsCommandHandler;
    let handler: Handler = Handler::with_command_handlers(vec![
        Box::new(update_settings_command_handler),
        Box::new(HelpCommandHandler),
        Box::new(UserSettingsCommandHandler),
        Box::new(RecipeCommandHandler)
    ]);
    let token = std::env::var("TOKEN").expect("TOKEN environment variable not set.");
    let mut client = serenity::Client::new(token, handler).expect("Error creating client");
    {
        let mut data = client.data.write();

        let file_name = "src/materials/materials.json";
        let json_file_contents = std::fs::read_to_string(file_name)
            .expect(&format!("Unable to find file {}", file_name));

        data.insert::<UserSettingsDatabase>(UserSettingsDatabase::new());
        data.insert::<MaterialDatabase>(MaterialDatabase::new(json_file_contents));
    }

    if let Err(why) = client.start() {
        println!("Client error: {:?}", why);
    }
}
