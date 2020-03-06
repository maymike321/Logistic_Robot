use crate::handlers::command_handler::CommandHandler;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

pub struct Handler {
    command_handlers: Vec<Box<dyn CommandHandler>>,
}

impl Handler {
    pub fn with_command_handlers(command_handlers: Vec<Box<dyn CommandHandler>>) -> Self {
        Handler { command_handlers }
    }
}

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        for command_handler in &self.command_handlers {
            command_handler.handle_command(&ctx, &msg);
        }
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}
