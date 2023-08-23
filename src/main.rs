mod commands;
mod util;

use songbird::SerenityInit;

use serenity::client::Context;

use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::{standard::macros::group, StandardFramework},
    model::gateway::Ready,
    prelude::GatewayIntents,
};

use commands::{
    deafen::*, join::*, leave::*, mute::*, neko::*, nurupo::*, pause::*, ping::*, play::*,
    queue::*, resume::*, saikoro::*, seek::*, skip::*, undeafen::*, unmute::*,
};

use crate::util::get_token;

use std::collections::HashSet;

use serenity::framework::standard::{
    help_commands, macros::help, Args, CommandGroup, CommandResult, HelpOptions,
};
use serenity::model::{channel::Message, id::UserId};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Bot起動時の処理
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[help] // Helpコマンド
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    // _ は使用しない返り値を捨てることを明示している
    let _ = help_commands::with_embeds(ctx, msg, args, help_options, groups, owners).await;
    // 空のタプルをreturn（仕様）
    // Rustでは`;`なしの行は値としてreturnすることを表す
    // return Ok(()); と同義
    Ok(())
}

#[group]
#[summary("音楽関連")]
#[commands(
    deafen,
    join,   // VCに参加
    leave,  // VCから退出
    mute,   // ミュート
    play,   // 音楽再生
    undeafen,
    unmute, // mute解除
    skip,   // スキップ
    pause, // 一時停止
    resume,// 一時停止解除
    seek,
    queue
)]

struct Music;
#[group]
#[summary("その他")]
#[commands(
    ping,   // ping-pong
    neko,   // 猫の鳴き声
    saikoro,
    nurupo,
)]

struct Other;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // トークンが記述されたconfigファイルを取得
    let token = get_token("config.json").expect("Err トークンが見つかりません");

    // フレームワーク
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~"))
        .help(&MY_HELP) // ヘルプコマンドを追加
        .group(&MUSIC_GROUP)
        .group(&OTHER_GROUP);

    // 特権とされていないintentとメッセージに関するintent
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;

    // Botのクライアントを作成
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .framework(framework)
        .register_songbird()
        .await
        .expect("Err creating client");

    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
    });

    // Ctrl+Cを検知した場合
    tokio::signal::ctrl_c().await.expect("");
    println!("Received Ctrl-C, shutting down.");
}
