use teloxide::{prelude::*, utils::command::BotCommands};

use std::error::Error;

use receipt_bot::db_interface::*;


#[derive(BotCommands, Clone)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "register the user")]
    Register,
    #[command(description = "get total balance")]
    GetBalance(String),
    #[command(description = "insert receipt from URL")]
    InsertFromUrl(String),
    #[command(description = "shut the bot down if you have the priviledges")]
    ShutDown
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

async fn answer (
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let con = Database::connect(&std::env::var("DATABASE_URL")?).expect("Error while connecting to db");

    match command {
        Command::Help => {
            bot.send_message(message.chat.id, Command::descriptions().to_string()).await?
        },
        Command::Register => {
            let res = con.get_user(message.chat.id.to_string().parse().unwrap()).expect("Could not fetch query!");
            
            let mut msg = "User already in the database!";
            if res.len() == 0 {
                con.insert_user(message.chat.id.to_string().parse().unwrap(), false).expect("Could not insert user");                
                msg = "User created!\nWelcome";
            }

            bot.send_message(message.chat.id, msg).await?
        },
        Command::GetBalance(month) => {
            bot.send_message(message.chat.id, Command::descriptions().to_string()).await?
        },
        Command::InsertFromUrl(url) => {
            bot.send_message(message.chat.id, Command::descriptions().to_string()).await?
        },
        Command::ShutDown => return Err(Box::new(teloxide::ApiError::Unknown("Shutdown Request".to_owned())))
    };

    Ok(())
}