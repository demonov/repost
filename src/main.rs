extern crate teleborg;
use teleborg::{Dispatcher, Bot, Updater};
use teleborg::objects::Update;

fn main() {
    let bot_token = "token".to_string();
    let mut dispatcher = Dispatcher::new();
    
    dispatcher.add_message_handler(msg_handler_fn);
    dispatcher.add_command_handler("debug", cmd_debug_fn, false);
    
    Updater::start(Some(bot_token), None, None, None, dispatcher);
}

fn cmd_debug_fn(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
    let text = format!("debug\n{:?}", args);
    if let Err(e) = bot.reply_to_message(&update, &text) {
        println!("Couldn't send debug message: {:?}", e);
    }   
}

fn msg_handler_fn(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
    match parse_message(&update) {
        Err(e) => println!("{}", e),

        Ok((chat_id, req_text)) => {
            let res_text = format!("{} -> response русский текст\n{:?}", req_text, args);
            if let Err(e) = bot.send_message(&chat_id, &res_text, None, None, None, None, None) {
                println!("Couldn't send message: {:?}", e);
            }
        }
    }
}

fn parse_message(update: &Update) -> Result<(i64, &str), &str> {
    match update.message {
        Some(ref message) => {
            let text = match message.text {
                Some(ref s) => &s,
                None => "",
            };

            Ok((message.chat.id, text))
        },
        None => Err("message not found"),
    }    
}