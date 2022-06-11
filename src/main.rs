use teloxide::{prelude::*, utils::command::BotCommands};

use std::error::Error;

use receipt_bot::{db_interface::*, web_scraper::Scraper};

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
    ShutDown,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::commands_repl(bot, answer, Command::ty()).await;
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let con = Database::connect(
        &std::env::var("DATABASE_URL")?,
    )
    .expect("Error while connecting to db");
    log::debug!("Connected to Database");

    log::info!("Handling command");
    match command {
        Command::Help => {
            log::info!("Print help message");
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Register => {
            log::info!("Register command run");
            let res = con
                .get_user(message.chat.id.to_string().parse().unwrap())
                .expect("Could not fetch query!");

            let mut msg = "User already in the database!";
            if res.len() == 0 {
                con.insert_user(message.chat.id.to_string().parse().unwrap(), false)
                    .expect("Could not insert user");
                msg = "User created!\nWelcome";
            }

            let tmp = bot.send_message(message.chat.id, msg).await?;
            log::info!("Preparing to send: '{}'", msg);
            tmp
        }
        Command::GetBalance(month) => {
            bot.send_message(message.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::InsertFromUrl(url) => {
            let scraper = Scraper::new(
                &url,
                message.chat.id.to_string().parse().unwrap_or_else(|_| {
                    log::error!("message.chat.id () could not be parsed to i64");
                    0
                }),
            )
            .await?;

            match con.insert_business(scraper.comp) {
                Ok(_) => log::info!("Inserted Company in DB"),
                Err(e) => log::error!("Could not insert Company in DB: {}", e),
            }

            match con.insert_employee(scraper.emp) {
                Ok(_) => log::info!("Inserted Employee in DB"),
                Err(e) => log::error!("Could not insert Employee in DB: {}", e),
            }

            let msg = match con.insert_receipt(scraper.receipt) {
                Ok(_) => {
                    log::info!("Inserted Receipt in DB");
                    "Receipt added to database!"
                },
                Err(e) => {
                    log::error!("Could not insert Receipt in DB: {}", e);
                    "An error occured, and could not insert receipt :("
                },
            };

            let a = bot
                .send_message(message.chat.id, msg)
                .await?;
            log::info!("Sent feedback message");
            a
        }
        Command::ShutDown => {
            log::info!("ShutDown command run");
            let res = con
                .get_user(message.chat.id.to_string().parse().unwrap())
                .expect("Could not run query!");

            if res.get(0).is_some() && res[0].is_admin.unwrap_or(false) {
                log::info!("Shutting down");
                bot.send_message(message.chat.id, "Shutting down...")
                    .await?;
                std::process::exit(0);
            }

            log::warn!(
                "Not admin tried to shutdown, {}",
                message.chat.id.to_string()
            );
            bot.send_message(message.chat.id, "You are not a admin!!!")
                .await?
        }
    };

    Ok(())
}
