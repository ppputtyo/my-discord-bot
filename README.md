# my-discord-bot

<img src="images/demo.gif">

[serenity](https://github.com/serenity-rs/serenity)と[songbird](https://github.com/serenity-rs/songbird)を利用した Rust 製音楽再生 bot です。

以下のコマンドを実行できます。

## 音楽関連

| コマンド    | 機能                                                      |
| ----------- | --------------------------------------------------------- |
| `~join`     | コマンドの実行者が参加しているボイスチャンネルに参加      |
| `~play URL` | URL の音楽を再生                                          |
| `~skip`     | 1 曲スキップ                                              |
| `~queue`    | 現在のキューを表示                                        |
| `~seek sec` | 引数で指定された秒数音楽をスキップ (引数が負の場合は戻る) |
| `~leave`    | ボイスチャンネルから退出                                  |
| `~pause`    | 再生中の音楽を一時停止                                    |
| `~resume`   | 一時停止解除                                              |
| `~mute`     | ミュート状態にする                                        |
| `~unmute`   | ミュート状態を解除                                        |
| `~deafen`   | スピーカーミュート状態にする                              |
| `~undeafen` | スピーカーミュート状態を解除                              |

## その他

| コマンド        | 機能                                                      |
| --------------- | --------------------------------------------------------- |
| `~saikoro [num]` | 1\~num の間の数値をランダムに返す (引数なしの場合は 1\~6) |
| `~ping`          | pong を返す                                               |
| `~neko`          | にゃーんを返す                                            |
| `~nurupo`        | ガッを返す                                                |

## 使用方法

1. yt-dlp、ffmpeg をインストール
2. discord bot に登録して APIキー を取得
3. このリポジトリを clone して`config_template.json`と同じ形式の`config.json`を作成して APIキー を貼り付け
4. `cargo run --release`

yt-dlpとffmpeg のインストール、discord bot の登録方法は[こちらの Qitta の記事](https://qiita.com/ppputtyo/items/bf95c9ccdba3b6042031)で詳しく説明しています。

## 動作確認済み環境
- Windows 11
- Ubuntu 23.04

(私のM1 MacではLLVMのリンカ周りでエラーが出て実行できませんでした。)
