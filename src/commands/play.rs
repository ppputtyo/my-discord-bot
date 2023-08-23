use std::{sync::Arc, time::Duration};

use crate::{
    commands::join::join,
    util::{check_msg, display_queue, SeekBar},
};

use serenity::{
    async_trait,
    framework::standard::{macros::command, Args, CommandResult},
    http::Http,
    model::prelude::{ChannelId, Message, MessageId},
    prelude::Context,
};
use songbird::{
    create_player, input::Restartable, Event, EventContext, EventHandler as VoiceEventHandler,
};

struct PeriodicNotifier {
    chan_id: ChannelId,
    http: Arc<Http>,
    message_id: MessageId,
    progress_bar: SeekBar,
}

#[async_trait]
impl VoiceEventHandler for PeriodicNotifier {
    async fn act(&self, ctx: &EventContext<'_>) -> Option<Event> {
        if let EventContext::Track([(track_state, _handler)]) = ctx {
            check_msg(
                self.chan_id
                    .edit_message(&self.http, self.message_id, |m| {
                        m.content(self.progress_bar.display(track_state.position))
                    })
                    .await,
            );
        }

        None
    }
}

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
    if manager.get(guild_id).is_none() {
        join(ctx, msg, args).await.expect("VC接続失敗");
    }

    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;
        let send_http = ctx.http.clone();

        let source = match Restartable::ytdl(url, true).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

                return Ok(());
            }
        };

        let (mut audio, audio_handler) = create_player(source.into());
        audio.set_volume(0.05);

        // 排他的に音楽再生
        handler.enqueue(audio);

        msg.channel_id
            .say(&send_http, display_queue(handler.queue()))
            .await
            .unwrap();

        let progress_bar = SeekBar::from(audio_handler.metadata());

        let message = msg.channel_id.say(&send_http, "```\n```").await.unwrap();

        let _ = audio_handler.add_event(
            Event::Periodic(Duration::from_secs(1), None),
            PeriodicNotifier {
                chan_id: msg.channel_id,
                http: send_http.clone(),
                message_id: message.id,
                progress_bar,
            },
        );
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Not in a voice channel to play in")
                .await,
        );
    }

    Ok(())
}
