use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Resource {
    artwork_url: Option<String>,
    avatar_url: Option<String>,
    created_at: Option<String>,
    duration: Option<i64>,
    followers_count: Option<i64>,
    followings_count: Option<i64>,
    full_duration: Option<i64>,
    genre: Option<String>,
    id: i64,
    kind: ResourceKind,
    likes_count: Option<i64>,
    permalink_url: Option<String>,
    playback_count: Option<i64>,
    tag_list: Option<String>,
    title: Option<String>,
    tracks: Option<Vec<Resource>>,
    media: Option<Media>,
    user: Option<Box<Resource>>,
    username: Option<String>,
}

impl Resource {
    pub fn artwork_url(&self) -> Option<String> {
        self.artwork_url.clone()
    }

    pub fn avatar_url(&self) -> Option<String> {
        self.avatar_url.clone()
    }

    pub fn created_at(&self) -> Option<String> {
        self.created_at.clone()
    }

    pub fn duration(&self) -> Option<i64> {
        self.duration.clone()
    }

    pub fn followers_count(&self) -> Option<i64> {
        self.followers_count.clone()
    }

    pub fn followings_count(&self) -> Option<i64> {
        self.followings_count.clone()
    }

    pub fn full_duration(&self) -> Option<i64> {
        self.full_duration.clone()
    }

    pub fn genre(&self) -> Option<String> {
        self.genre.clone()
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn kind(&self) -> ResourceKind {
        self.kind
    }

    pub fn likes_count(&self) -> Option<i64> {
        self.likes_count
    }

    pub fn permalink_url(&self) -> Option<String> {
        self.permalink_url.clone()
    }

    pub fn playback_count(&self) -> Option<i64> {
        self.playback_count.clone()
    }

    pub fn tag_list(&self) -> Option<String> {
        self.tag_list.clone()
    }

    pub fn title(&self) -> Option<String> {
        self.title.clone()
    }

    pub fn tracks(&self) -> Option<Vec<Resource>> {
        self.tracks.clone()
    }

    pub fn media(&self) -> Option<Media> {
        self.media.clone()
    }

    pub fn user(&self) -> Option<Resource> {
        self.user.clone().map(|user| *user)
    }

    pub fn username(&self) -> Option<String> {
        self.username.clone()
    }
}

#[derive(Debug, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ResourceKind {
    Track,
    User,
    Playlist,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Media {
    transcodings: Vec<Transcoding>,
}

impl Media {
    pub fn transcodings(&self) -> Vec<Transcoding> {
        self.transcodings.clone()
    }

    pub fn progressive(&self) -> Option<Transcoding> {
        self.transcodings()
            .into_iter()
            .find(|transcoding| transcoding.format().protocol().to_string() == "progressive")
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Transcoding {
    url: String,
    preset: String,
    duration: i64,
    format: TranscodingFormat,
    quality: String,
}

impl Transcoding {
    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn preset(&self) -> String {
        self.preset.clone()
    }

    pub fn duration(&self) -> i64 {
        self.duration
    }

    pub fn format(&self) -> TranscodingFormat {
        self.format.clone()
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct TranscodingFormat {
    protocol: String,
    mime_type: String,
}

impl TranscodingFormat {
    pub fn protocol(&self) -> String {
        self.protocol.clone()
    }

    pub fn mime_type(&self) -> String {
        self.mime_type.clone()
    }

    pub fn file_extension(&self) -> FileExtension {
        if self.mime_type.starts_with("audio/mp4; codecs=\"mp4a\"") {
            FileExtension::M4A
        } else {
            FileExtension::MP3
        }
    }
}

pub enum FileExtension {
    M4A,
    MP3,
}

impl std::fmt::Display for FileExtension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Self::M4A => write!(f, ".m4a"),
            Self::MP3 => write!(f, ".mp3"),
        }
    }
}
