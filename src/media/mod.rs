use serde::Deserialize;
use std::path::Path;

mod animated_gif;
mod photo;
mod video;
pub use animated_gif::AnimatedGif;
pub use photo::Photo;
pub use video::Video;
use crate::sql::UpdateMediaSql;

#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Media {
    AnimatedGif(AnimatedGif),
    Photo(Photo),
    Video(Video),
}

impl Media {
    pub fn r#type(&self) -> &str {
        match self {
            Media::AnimatedGif(_) => "animated_gif",
            Media::Photo(_) => "photo",
            Media::Video(_) => "video",
        }
    }

    fn file_name(&self) -> String {
        match self {
            Media::AnimatedGif(gif) => gif.file_name(),
            Media::Photo(photo) => photo.file_name(),
            Media::Video(video) => video.file_name(),
        }
    }

    pub fn archive_file_name<P: AsRef<Path>>(&self, prefix: P) -> String {
        let prefix = prefix.as_ref().display();
        let suffix = match self {
            Media::AnimatedGif(gif) => gif.file_name(),
            Media::Photo(photo) => photo.file_name(),
            Media::Video(video) => video.file_name(),
        };

        format!("{prefix}-{suffix}")
    }

    pub fn update_sql(&self) -> UpdateMediaSql {
        UpdateMediaSql::default()
    }
}

#[derive(Clone, Debug, Deserialize)]
struct VideoInfo {
    aspect_ratio: [String; 2],
    duration_millis: Option<String>,
    variants: Vec<Variant>,
}

impl VideoInfo {
    fn highest_bitrate_variant(&self) -> Option<Variant> {
        let mut selected_variant: Option<Variant> = None;
        let mut highest_bitrate = 0;
        for variant in &self.variants {
            if let Some(bitrate) = &variant.bitrate {
                if let Ok(bitrate) = bitrate.parse() {
                    if bitrate >= highest_bitrate {
                        highest_bitrate = bitrate;
                        selected_variant = Some(variant.clone());
                    }
                }
            }
        }

        selected_variant
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Variant {
    bitrate: Option<String>,
    content_type: String,
    url: String,
}

#[derive(Clone, Debug, Deserialize)]
struct Sizes {
    medium: Size,
    large: Size,
    thumb: Size,
    small: Size,
}

#[derive(Clone, Debug, Deserialize)]
struct Size {
    w: String,
    h: String,
    resize: String,
}
