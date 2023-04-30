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

    // ハンドラを持っているかどうか
    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
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
    } else {
        check_msg(msg.reply(ctx, "ボイスチャンネルに入ってないよ").await);
    }

    Ok(())
}
