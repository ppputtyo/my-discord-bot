use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::util::check_msg;

#[command]
#[description = "猫のように鳴く"]
async fn neko(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(
        msg.channel_id
            .say(&ctx.http, format!("{} にゃーん", msg.author.mention()))
            .await,
    );
    Ok(())
}
