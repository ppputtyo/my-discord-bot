use serde::{Deserialize, Serialize};
use serde_json::Result;
use serenity::{model::prelude::Message, Result as SerenityResult};
use songbird::{input::Metadata, tracks::TrackQueue};
use std::{fs::File, io::BufReader, time::Duration};

#[derive(Serialize, Deserialize)]
struct Token {
    token: String,
}

pub(crate) fn get_token(file_name: &str) -> Result<String> {
    let file = File::open(file_name).unwrap();
    let reader = BufReader::new(file);
    let t: Token = serde_json::from_reader(reader).unwrap();
    Ok(t.token)
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
pub(crate) fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

// XX:XX
pub fn display_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();

    format!("{:0>2}:{:0>2}", seconds / 60, seconds % 60)
}

/// シークバー
pub struct SeekBar {
    track_name: String,
    /// 最大値
    max: Duration,
}

impl From<&Metadata> for SeekBar {
    fn from(metadata: &Metadata) -> Self {
        Self {
            track_name: metadata.title.as_ref().unwrap().clone(),
            max: metadata.duration.unwrap(),
        }
    }
}

impl SeekBar {
    pub fn display(&self, current: Duration) -> String {
        let mut progress = current.as_millis() as f32 / self.max.as_millis() as f32 * 100.0;
        if progress > 100.0 {
            progress = 100.0;
        }

        let progress_bar = format!(
            "# {}\n```\n[{}|{}]\n{} / {}\n```",
            self.track_name,
            "─".repeat((progress) as usize),
            "─".repeat(100 - (progress) as usize),
            display_duration(current),
            display_duration(self.max)
        );

        progress_bar
    }
}

pub fn display_queue(queue: &TrackQueue) -> String {
    let track_handles = queue.current_queue();

    let mut res = String::new();

    for (i, handle) in track_handles.iter().enumerate() {
        let title = handle.metadata().title.as_ref().unwrap();
        let url = handle.metadata().source_url.as_ref().unwrap();
        if i == 1 {
            res.push('\n');
        }

        if i == 0 {
            res.push_str("playing: ");
        } else {
            res.push_str(&format!("{:<2}: ", i));
        }

        res.push_str(&format!("[{}]({})\n", title, url));
    }

    res
}
