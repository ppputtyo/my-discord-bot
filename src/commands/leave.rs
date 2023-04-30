use std::fs;

use crate::util::check_msg;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let queue = handler.queue();
        let _ = queue.stop();

        check_msg(msg.channel_id.say(&ctx.http, "キューをクリアしたよ").await);

        // 切断を試みる
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }

        check_msg(
            msg.channel_id
                .say(&ctx.http, "ボイスチャンネルから切断しました")
                .await,
        );

        // audioディレクトリを削除
        fs::remove_dir_all("audio").unwrap_or_else(|why| {
            println!("{}", why);
        });
    } else {
        check_msg(msg.reply(ctx, "ボイスチャンネルに入ってないよ").await);
    }

    Ok(())
}
