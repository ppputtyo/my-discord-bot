use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::process::Command;

#[command]
#[description = "音楽ダウンロード"]
async fn dlmusic(ctx: &Context, msg: &Message) -> CommandResult {
    let url: Vec<&str> = msg.content.split_whitespace().collect();

    let output = Command::new("yt-dlp")
        .args(&[
            "-o",
            "/Users/ppputtyo/Documents/GitHub/my-discord-bot/videos/1.mp3",
            "-x",
            "--audio-format",
            "mp3",
            // "--output",
            // "audio.mp3",
            // "--extract-audio",
            &format!("{}", url[1]),
        ])
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Audio downloaded successfully");
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        println!("Error: {}", error);
    }

    // msg.channel_id.say で，channel_id の channel にメッセージを投稿
    msg.channel_id
        .say(&ctx.http, format!("{} にゃーん", msg.author.mention()))
        .await?;
    // CommandResultはResultを継承している
    // `Result?` は正常な値の場合，Resultの中身を返し，エラーの場合は即座にreturnする演算子
    Ok(())
}
