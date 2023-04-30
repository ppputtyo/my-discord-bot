use crate::util::check_msg;

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};

#[command]
#[only_in(guilds)]
async fn resume(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;

        let queue = handler.queue();

        let _ = queue.resume();

        check_msg(msg.channel_id.say(&ctx.http, "一時停止解除").await);
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "ボイスチャンネルに入ってないよ")
                .await,
        );
    }

    Ok(())
}
