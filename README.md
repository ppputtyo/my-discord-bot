# my-discord-bot


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
| `saikoro [num]` | 0\~num の間の数値をランダムに返す (引数なしの場合は 0\~6) |
| `ping`          | pong を返す                                               |
| `neko`          | にゃーんを返す                                            |
| `nurupo`        | ガッを返す                                                |

## 使用方法
1. yt-dlp、ffmpegをインストール
2. discord bot登録してAPIを取得
3. このリポジトリをcloneして`config_template.json`と同じ形式の`config.json`を作成してAPIを貼り付け
4. リポジトリ実行

yt-dlp,ffmpegのインストール、discord botの登録方法などは[こちらのQittaの記事]で詳しく説明しています。
