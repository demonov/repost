extern crate teleborg;
use teleborg::{Dispatcher, Bot, Updater, NO_MARKUP};
use teleborg::objects::{InlineKeyboardButton, InlineKeyboardMarkup, Message, Update};

fn main() {
    let mut dispatcher = Dispatcher::new();
    
    dispatcher.add_message_handler(msg_handler_fn);
    dispatcher.add_command_handler("debug", cmd_debug_fn, false);
    
    Updater::start(None, None, None, None, dispatcher);
}

fn cmd_debug_fn(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
    let text = format!("debug\n{:?}", args);
    if let Err(e) = bot.reply_to_message(&update, &text) {
        println!("Couldn't send debug message: {:?}", e);
    }
}

fn msg_handler_fn(bot: &Bot, update: Update, args: Option<Vec<&str>>) {
    if let Some(ref callback_query) = update.callback_query {
        match parse_message(&callback_query.message) {
            Err(e) => println!("{}", e),
            Ok((chat_id, req_text)) => {
                if let Some(ref res_text) = callback_query.data {
                    if let Err(e) = bot.send_message(&chat_id, &res_text, None, None, None, None, NO_MARKUP) {
                        println!("Couldn't send message: {:?}", e);
                    }
                } else {
                    println!("Couldn't parse callback_query data");
                }
            }
        }
    } else {
        match parse_message(&update.message) {
            Err(e) => println!("{}", e),
            Ok((chat_id, req_text)) => {
                let res_text = format!("{} -> response русский текст\n{:?}", req_text, args);

                let button = InlineKeyboardButton::new("Title".to_string(), None, Some(String::from("abc")), None, None);
                let markup = InlineKeyboardMarkup::new(vec![vec![button]]);

                if let Err(e) = bot.send_message(&chat_id, &res_text, None, None, None, None, Some(markup)) {
                    println!("Couldn't send message: {:?}", e);
                }

            }
        }
    }
}

fn parse_message<'a>(msg:&'a Option<Message>) -> Result<(i64, &'a str), &'a str> {
    if let &Some(ref message) = msg {
        let text = match message.text {
                Some(ref s) => s,
                None => "",
        };

        return Ok((message.chat.id, text))
    }
    
    Err("message not found")
}