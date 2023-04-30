use serde::{Deserialize, Serialize};
use serde_json::Result;
use serenity::{model::prelude::Message, Result as SerenityResult};
use std::{fs::File, io::BufReader};

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

pub(crate) fn get_token(file_name: &str) -> Result<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let t: Token = serde_json::from_reader(reader).unwrap();
    Ok(t.token)
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
pub(crate) fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
