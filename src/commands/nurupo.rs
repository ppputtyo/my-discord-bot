use crate::util::check_msg;

use serenity::{
    framework::standard::{macros::command, CommandResult},
    model::prelude::Message,
    prelude::{Context, Mentionable},
};
#[command]
async fn nurupo(context: &Context, msg: &Message) -> CommandResult {
    let res = format!(
        r"{}
```
　　 （　・∀・）　　　|　|　ｶﾞｯ
　　と　　　　）　 　 |　|
　　　 Ｙ　/ノ　　　 人
　　　　 /　）　 　 < 　>__Λ∩
　　 ＿/し'　／／. Ｖ｀Д´）/
　　（＿フ彡　　　　　 　　/
```",
        msg.author.mention()
    );
    check_msg(msg.channel_id.say(&context.http, res).await);

    Ok(())
}
