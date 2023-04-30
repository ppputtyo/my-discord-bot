use std::{process::Command, time::Instant};

use crate::{commands::join::join, util::check_msg};

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};
use songbird::{create_player, ffmpeg};

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        println!("{:#?}", queue);

        let len = queue.len();

        if len == 0 {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("スキップする曲がないよ"))
                    .await,
            );
            return Ok(());
        }

        let _ = queue.skip();

        check_msg(
            msg.channel_id
                .say(
                    &ctx.http,
                    format!("スキップ成功: あと{}曲残ってるよ ", len - 1),
                )
                .await,
        );
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "ボイスチャンネルに入ってないよ")
                .await,
        );
    }

    Ok(())
}
