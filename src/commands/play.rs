use crate::{commands::join::join, util::check_msg};

use serenity::{
    framework::standard::{macros::command, Args, CommandResult},
    model::prelude::Message,
    prelude::Context,
};
use songbird::{create_player, input::Restartable, tracks::Track};

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    // 引数からURLを取得
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(msg.channel_id.say(&ctx.http, "URL頂戴").await);

            return Ok(());
        }
    };

    // httpから始まらない場合はエラー
    if !url.starts_with("http") {
        check_msg(msg.channel_id.say(&ctx.http, "httpから始まるURL頂戴").await);

        return Ok(());
    }

    // サーバ情報の取得
    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("初期化できてないよ")
        .clone();

    // VCに接続していない場合は接続
    if let None = manager.get(guild_id) {
        join(ctx, msg, args).await.expect("VC接続失敗");
    }

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match Restartable::ytdl(url, true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

                return Ok(());
            }
        };

        let (mut audio, _) = create_player(source.into());
        audio.set_volume(0.05);

        // 排他的に音楽再生
        handler.enqueue(audio);

        let queue_len = handler.queue().len();
        if queue_len == 1 {
            check_msg(msg.channel_id.say(&ctx.http, "再生中～～").await);
        } else {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("{}曲後に再生されるよ", queue_len - 1))
                    .await,
            );
        }
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}
