use std::time::Duration;

use crate::util::check_msg;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
#[only_in(guilds)]
async fn seek(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let seek_sec = match args.single::<String>() {
        Ok(arg) => match arg.parse::<i128>() {
            Ok(num) => num,
            Err(_) => {
                check_msg(
                    msg.channel_id
                        .say(&ctx.http, "シークする時間を秒数で指定してね")
                        .await,
                );
                return Ok(());
            }
        },
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "引数としてシークする時間を秒数で指定してね")
                    .await,
            );
            return Ok(());
        }
    };

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let current = handler.queue().current().unwrap();

        let info = current.get_info().await.unwrap();
        let metadata = current.metadata();

        let current_position = info.position;
        let duration_limit = metadata.duration.unwrap().as_millis() as i128;

        let new_seek_position = (current_position.as_millis() as i128 + seek_sec * 1000)
            .max(0)
            .min(duration_limit) as u64;

        current
            .seek_time(Duration::from_millis(new_seek_position))
            .expect("Failed to seek time");

        if seek_sec < 0 {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("{}秒巻き戻したよ", seek_sec.abs()))
                    .await,
            );
        } else {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("{}秒進めたよ", seek_sec))
                    .await,
            );
        }
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "ボイスチャンネルに入ってないよ")
                .await,
        );
    }

    Ok(())
}
