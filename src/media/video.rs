use std::{
    path::PathBuf,
};
use serde::Deserialize;
use super::{VideoInfo, Sizes};

#[derive(Clone, Debug, Deserialize)]
pub struct Video {
    id_str: String,
    url: String,
    expanded_url: String,
    indices: Option<[String; 2]>,
    media_url: String,
    media_url_https: String,
    display_url: String,
    source_status_id: Option<String>,
    source_status_id_str: Option<String>,
    video_info: Option<VideoInfo>,
    source_user_id: Option<String>,
    source_user_id_str: Option<String>,
    additional_media_info: Option<AdditionalMediaInfo>,
    sizes: Sizes,
}

#[derive(Clone, Debug, Deserialize)]
struct AdditionalMediaInfo {
    title: Option<String>,
    description: Option<String>,
    embeddable: Option<bool>,
    monetizable: bool,
}

impl Video {
    pub fn id(&self) -> u64 {
        self.id_str.parse().unwrap()
    }

    pub fn file_name(&self) -> String {
        let Some(info) = &self.video_info else {
            todo!("{:#?}", self)
        };

        let Some(variant) = info.highest_bitrate_variant() else {
            todo!("{:#?}", self)
        };

        let path = PathBuf::from(&variant.url);
        let mut path = path.file_name().unwrap().to_string_lossy().to_string();

        // Remove query strings
        if let Some(index) = path.find('?') {
            path = path[..index].into();
        }
        path
    }
}
