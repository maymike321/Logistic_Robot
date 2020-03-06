use serenity::{model::channel::Message, prelude::*};

pub trait CommandHandler: Send + Sync {
    fn handle_command(&self, context: &Context, message: &Message);
}
