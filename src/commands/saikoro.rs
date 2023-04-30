use rand::Rng;
use serenity::framework::standard::Args;
use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::util::check_msg;

#[command]
#[only_in(guilds)]
async fn saikoro(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let limit = match args.single::<String>() {
        Ok(arg) => match arg.parse() {
            Ok(num) => num,
            Err(_) => return Ok(()),
        },
        Err(_) => 6,
    };

    let res = {
        let mut rng = rand::thread_rng();
        rng.gen::<usize>() % limit + 1
    };

    check_msg(
        msg.channel_id
            .say(&ctx.http, format!("結果: {}", res))
            .await,
    );
    Ok(())
}
