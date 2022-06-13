use teloxide::{
    dispatching::{update_listeners, UpdateFilterExt},
    net::Download,
    prelude::*,
    types::File,
    utils::command::BotCommands, dptree::di::{Injectable, Asyncify},
};
use tokio::fs::File as TFile;

use std::error::Error;

use receipt_bot::{db_interface::*, qr_reader::read_url_from_qr, web_scraper::Scraper};

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
    Dispatcher::builder(
        bot.clone(),
        Update::filter_message()
            .branch(dptree::entry().filter_command::<Command>().endpoint(answer))
            .chain(dptree::entry().endpoint(answer_photo)),
    )
    .default_handler(|_upd| Box::pin(async {}))
    .build()
    .setup_ctrlc_handler()
    .dispatch_with_listener(
        update_listeners::polling_default(bot).await,
        LoggingErrorHandler::with_custom_text("Custom Dispatcher not working!"),
    )
    .await;
}

async fn answer(
    bot: AutoSend<Bot>,
    message: Message,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let con =
        Database::connect(&std::env::var("DATABASE_URL")?).expect("Error while connecting to db");
    log::info!("Connected to Database");

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

            let msg = insert_scraped_data(&con, scraper);
            let a = bot.send_message(message.chat.id, msg).await?;
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

async fn answer_photo(
    bot: AutoSend<Bot>,
    message: Message,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    log::debug!(
        "message.text(): {:?}, message.photo(): {:?}",
        message.text().is_some(),
        message.photo().is_some()
    );

    if let Some(p) = message.photo() {
        let con = Database::connect(&std::env::var("DATABASE_URL")?)
            .expect("Error while connecting to db");
        log::info!("Succesful connection to Database");

        let File { file_path, .. } = bot.get_file(p[p.len() - 1].file_id.clone()).send().await?;
        log::info!("Got file succesfully");

        let file_name = format!(
            "./tmp/{}.jpg",
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );

        let mut file = TFile::create(&file_name).await?;

        bot.download_file(&file_path, &mut file).await?;

        let msg = match read_url_from_qr(&file_name) {
            Ok(u) => {
                let scraper = Scraper::new(
                    &u,
                    message.chat.id.to_string().parse().unwrap_or_else(|_| {
                        log::error!("message.chat.id () could not be parsed to i64");
                        0
                    }),
                ).await?;

                insert_scraped_data(&con, scraper)
            }
            Err(e) => {
                log::error!("Invalid photo uploaded");
                "The first photo you uploaded was not recognied as a qr code!"
            }
        };

        bot.send_message(message.chat.id, msg).await?;
        log::info!("Sent feedback message");
    }

    Ok(())
}

fn insert_scraped_data(con: &Database, scraper: Scraper) -> &'static str {
    match con.insert_business(scraper.comp) {
        Ok(_) => log::info!("Inserted Company in DB"),
        Err(e) => log::error!("Could not insert Company in DB: {}", e),
    }

    match con.insert_employee(scraper.emp) {
        Ok(_) => log::info!("Inserted Employee in DB"),
        Err(e) => log::error!("Could not insert Employee in DB: {}", e),
    }

    match con.insert_receipt(scraper.receipt) {
        Ok(_) => {
            log::info!("Inserted Receipt in DB");
            "Receipt added to database!"
        }
        Err(e) => {
            log::error!("Could not insert Receipt in DB: {}", e);
            "An error occured, and could not insert receipt :("
        }
    }
}

// impl<Func, Input, Output> Injectable<Input, Output, ()> for Asyncify<>