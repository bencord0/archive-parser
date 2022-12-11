use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Photo {
    pub id_str: String,
    pub url: String,
    pub expanded_url: String,
    pub indices: Option<[String; 2]>,
    pub media_url: String,
    pub media_url_https: String,
    pub display_url: String,
}

impl Photo {
    pub fn id(&self) -> u64 {
        self.id_str.parse().unwrap()
    }

    pub fn file_name(&self) -> String {
        let path = std::path::PathBuf::from(&self.media_url);
        let f = path.file_name().unwrap().to_string_lossy().to_string();

        if !(f.ends_with(".jpg") || f.ends_with(".png")) {
            panic!("not an image");
        }
        f
    }
}
