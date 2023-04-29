use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

use serenity::{framework::standard::Args, model::channel::Message, Result as SerenityResult};
use songbird::SerenityInit;

#[command]
#[description = "再生"]
async fn playmusic(ctx: &Context, msg: &Message) -> CommandResult {
    // msg.channel_id.say で，channel_id の channel にメッセージを投稿
    msg.channel_id
        .say(&ctx.http, format!("{} にゃーん", msg.author.mention()))
        .await?;
    // CommandResultはResultを継承している
    // `Result?` は正常な値の場合，Resultの中身を返し，エラーの場合は即座にreturnする演算子
    Ok(())
}

// #[command]
// #[only_in(guilds)]
// async fn deafen(ctx: &Context, msg: &Message) -> CommandResult {
//     let guild = msg.guild(&ctx.cache).unwrap();
//     let guild_id = guild.id;

//     let manager = songbird::get(ctx)
//         .await
//         .expect("Songbird Voice client placed in at initialisation.")
//         .clone();

//     let handler_lock = match manager.get(guild_id) {
//         Some(handler) => handler,
//         None => {
//             check_msg(msg.reply(ctx, "Not in a voice channel").await);

//             return Ok(());
//         }
//     };

//     let mut handler = handler_lock.lock().await;

//     if handler.is_deaf() {
//         check_msg(msg.channel_id.say(&ctx.http, "Already deafened").await);
//     } else {
//         if let Err(e) = handler.deafen(true).await {
//             check_msg(
//                 msg.channel_id
//                     .say(&ctx.http, format!("Failed: {:?}", e))
//                     .await,
//             );
//         }

//         check_msg(msg.channel_id.say(&ctx.http, "Deafened").await);
//     }

//     Ok(())
// }

// #[command]
// #[only_in(guilds)]
// async fn join(ctx: &Context, msg: &Message) -> CommandResult {
//     let guild = msg.guild(&ctx.cache).unwrap();
//     let guild_id = guild.id;

//     let channel_id = guild
//         .voice_states
//         .get(&msg.author.id)
//         .and_then(|voice_state| voice_state.channel_id);

//     let connect_to = match channel_id {
//         Some(channel) => channel,
//         None => {
//             check_msg(msg.reply(ctx, "Not in a voice channel").await);

//             return Ok(());
//         }
//     };

//     let manager = songbird::get(ctx)
//         .await
//         .expect("Songbird Voice client placed in at initialisation.")
//         .clone();

//     let _handler = manager.join(guild_id, connect_to).await;

//     Ok(())
// }

// #[command]
// #[only_in(guilds)]
// async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
//     let guild = msg.guild(&ctx.cache).unwrap();
//     let guild_id = guild.id;

//     let manager = songbird::get(ctx)
//         .await
//         .expect("Songbird Voice client placed in at initialisation.")
//         .clone();
//     let has_handler = manager.get(guild_id).is_some();

//     if has_handler {
//         if let Err(e) = manager.remove(guild_id).await {
//             check_msg(
//                 msg.channel_id
//                     .say(&ctx.http, format!("Failed: {:?}", e))
//                     .await,
//             );
//         }

//         check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);
//     } else {
//         check_msg(msg.reply(ctx, "Not in a voice channel").await);
//     }

//     Ok(())
// }

// #[command]
// #[only_in(guilds)]
// async fn mute(ctx: &Context, msg: &Message) -> CommandResult {
//     let guild = msg.guild(&ctx.cache).unwrap();
//     let guild_id = guild.id;

//     let manager = songbird::get(ctx)
//         .await
//         .expect("Songbird Voice client placed in at initialisation.")
//         .clone();

//     let handler_lock = match manager.get(guild_id) {
//         Some(handler) => handler,
//         None => {
//             check_msg(msg.reply(ctx, "Not in a voice channel").await);

//             return Ok(());
//         }
//     };

//     let mut handler = handler_lock.lock().await;

//     if handler.is_mute() {
//         check_msg(msg.channel_id.say(&ctx.http, "Already muted").await);
//     } else {
//         if let Err(e) = handler.mute(true).await {
//             check_msg(
//                 msg.channel_id
//                     .say(&ctx.http, format!("Failed: {:?}", e))
//                     .await,
//             );
//         }

//         check_msg(msg.channel_id.say(&ctx.http, "Now muted").await);
//     }

//     Ok(())
// }

#[command]
async fn play(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let url = match args.single::<String>() {
        Ok(url) => url,
        Err(_) => {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Must provide a URL to a video or audio")
                    .await,
            );

            return Ok(());
        }
    };

    if !url.starts_with("http") {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Must provide a valid URL")
                .await,
        );

        return Ok(());
    }

    let guild = msg.guild(&ctx.cache).unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx);
    // let manager = songbird::get(ctx)
    //     .await
    //     .expect("Songbird Voice client placed in at initialisation.")
    //     .clone();

    // if let Some(handler_lock) = manager.get(guild_id) {
    //     let mut handler = handler_lock.lock().await;

    //     let source = match songbird::ytdl(&url).await {
    //         Ok(source) => source,
    //         Err(why) => {
    //             println!("Err starting source: {:?}", why);

    //             check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

    //             return Ok(());
    //         }
    //     };

    //     handler.play_source(source);

    //     check_msg(msg.channel_id.say(&ctx.http, "Playing song").await);
    // } else {
    //     check_msg(
    //         msg.channel_id
    //             .say(&ctx.http, "Not in a voice channel to play in")
    //             .await,
    //     );
    // }

    Ok(())
}

// #[command]
// #[only_in(guilds)]
// async fn undeafen(ctx: &Context, msg: &Message) -> CommandResult {
//     let guild = msg.guild(&ctx.cache).unwrap();
//     let guild_id = guild.id;

//     let manager = songbird::get(ctx)
//         .await
//         .expect("Songbird Voice client placed in at initialisation.")
//         .clone();

//     if let Some(handler_lock) = manager.get(guild_id) {
//         let mut handler = handler_lock.lock().await;
//         if let Err(e) = handler.deafen(false).await {
//             check_msg(
//                 msg.channel_id
//                     .say(&ctx.http, format!("Failed: {:?}", e))
//                     .await,
//             );
//         }

//         check_msg(msg.channel_id.say(&ctx.http, "Undeafened").await);
//     } else {
//         check_msg(
//             msg.channel_id
//                 .say(&ctx.http, "Not in a voice channel to undeafen in")
//                 .await,
//         );
//     }

//     Ok(())
// }

// #[command]
// #[only_in(guilds)]
// async fn unmute(ctx: &Context, msg: &Message) -> CommandResult {
//     let guild = msg.guild(&ctx.cache).unwrap();
//     let guild_id = guild.id;

//     let manager = songbird::get(ctx)
//         .await
//         .expect("Songbird Voice client placed in at initialisation.")
//         .clone();

//     if let Some(handler_lock) = manager.get(guild_id) {
//         let mut handler = handler_lock.lock().await;
//         if let Err(e) = handler.mute(false).await {
//             check_msg(
//                 msg.channel_id
//                     .say(&ctx.http, format!("Failed: {:?}", e))
//                     .await,
//             );
//         }

//         check_msg(msg.channel_id.say(&ctx.http, "Unmuted").await);
//     } else {
//         check_msg(
//             msg.channel_id
//                 .say(&ctx.http, "Not in a voice channel to unmute in")
//                 .await,
//         );
//     }

//     Ok(())
// }

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
