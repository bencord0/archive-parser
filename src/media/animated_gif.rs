use std::path::PathBuf;
use serde::Deserialize;
use super::{Sizes, VideoInfo};

#[derive(Clone, Debug, Deserialize)]
pub struct AnimatedGif {
    pub id_str: String,
    pub url: String,
    pub expanded_url: String,
    pub indices: Option<[String; 2]>,
    pub media_url: String,
    pub media_url_https: String,
    pub display_url: String,
    sizes: Sizes,
    video_info: VideoInfo,
}

impl AnimatedGif {
    pub fn id(&self) -> u64 {
        self.id_str.parse().unwrap()
    }

    pub fn file_name(&self) -> String {
        let Some(variant) = self.video_info.highest_bitrate_variant() else {
            todo!("{:#?}", self)
        };

        let path = PathBuf::from(&variant.url);
        path.file_name().unwrap().to_string_lossy().to_string()
    }
}
