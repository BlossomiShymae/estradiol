use once_cell::sync::OnceCell;
use ureq::{Agent, AgentBuilder, Error, MiddlewareNext, Request, Response};

use crate::{
    endpoints::{
        get_bytes, get_client_id, get_search, get_stream, get_stream_bytes, get_track, SOUNDCLOUD,
    },
    models::{
        collections::Collection,
        media::Stream,
        resources::{Resource, Transcoding},
    },
};

const USER_AGENT: &str =
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:132.0) Gecko/20100101 Firefox/132.0";

pub(super) static AGENT: std::sync::LazyLock<Agent> = std::sync::LazyLock::new(|| {
    AgentBuilder::new()
        .middleware(
            |req: Request, next: MiddlewareNext| -> Result<Response, Error> {
                next.handle(
                    req.set("Origin", SOUNDCLOUD)
                        .set("Referer", SOUNDCLOUD)
                        .set("User-Agent", USER_AGENT),
                )
            },
        )
        .build()
});

#[derive(Debug)]
pub struct Client {
    agent: Agent,
    client_id: OnceCell<String>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            agent: AGENT.clone(),
            client_id: OnceCell::new(),
        }
    }

    pub fn client_id(&self) -> Result<&String, super::Error> {
        self.client_id
            .get_or_try_init(|| get_client_id(&self.agent))
    }

    pub fn track(&self, id: i64) -> Result<Resource, super::Error> {
        let client_id = self.client_id()?;
        get_track(&self.agent, client_id, id)
    }

    pub fn stream(&self, transcoding: &Transcoding) -> Result<Stream, super::Error> {
        let client_id = self.client_id()?;
        get_stream(&self.agent, client_id, transcoding)
    }

    pub fn bytes(&self, url: &str) -> Result<Vec<u8>, super::Error> {
        get_bytes(&self.agent, url)
    }

    pub fn search(&self, query: &str, limit: i64, offset: i64) -> Result<Collection, super::Error> {
        let client_id = self.client_id()?;
        get_search(&self.agent, client_id, query, limit, offset)
    }
}

impl Default for Client {
    fn default() -> Self {
        Client::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::Client;

    const TRACK_ID: i64 = 1_126_821_928; // BIG SHOT - Toby Fox

    #[test]
    fn test_track_stream_bytes() -> Result<(), crate::Error> {
        let client = Client::default();

        let track = client.track(TRACK_ID)?;
        let transcoding = track.media().unwrap().progressive().unwrap();
        let stream = client.stream(&transcoding)?;
        let stream_bytes = client.bytes(&stream.url())?;

        assert_eq!(track.title().unwrap(), String::from("BIG SHOT"));
        assert_eq!(
            track.user().unwrap().username().unwrap(),
            String::from("Toby Fox")
        );
        assert!(stream
            .url()
            .contains("https://cf-media.sndcdn.com/rbhlkHzpPAnQ.128.mp3"));
        assert!(!stream_bytes.is_empty());

        Ok(())
    }

    #[test]
    fn test_search() -> Result<(), crate::Error> {
        let client = Client::default();

        let search = client.search("undertale", 50, 0)?;

        assert!(!search.collection().is_empty());
        assert!(search.total_results().unwrap() > 200_000);
        assert!(search.next_href().is_some());
        assert!(search.query_urn().is_some());

        Ok(())
    }
}
