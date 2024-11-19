use regex::{Regex, RegexBuilder};
use serde::de::value::Error;
use ureq::Agent;

use crate::models::{
    collections::Collection,
    media::Stream,
    resources::{Resource, Transcoding},
};

pub(crate) const SOUNDCLOUD: &str = "https://soundcloud.com";
const SOUNDCLOUD_API_V2: &str = "https://api-v2.soundcloud.com";
const SEARCH: &str = "/search";
const TRACKS: &str = "/tracks/{id}";
const TRACKS_COMMENTS: &str = "/comments";
const TRACKS_RELATED: &str = "/related";
const TRACKS_ALBUMS: &str = "/albums";
const TRACKS_PLAYLISTS: &str = "/playlists";
const TRACKS_LIKERS: &str = "/likers";
const TRACKS_REPOSTERS: &str = "/reposters";

pub(crate) fn get_track(agent: &Agent, client_id: &str, id: i64) -> Result<Resource, super::Error> {
    let filename = TRACKS.replace("{id}", &id.to_string());
    let path = format!("{SOUNDCLOUD_API_V2}{filename}");

    let res = match agent
        .get(path.as_str())
        .query("client_id", client_id)
        .call()
    {
        Ok(res) => res,
        Err(err) => return Err(crate::Error::Ureq(Box::new(err))),
    };

    match res.into_json::<Resource>() {
        Ok(track) => Ok(track),
        Err(err) => Err(crate::Error::StdIo(err)),
    }
}

pub(crate) fn get_stream(
    agent: &Agent,
    client_id: &str,
    transcoding: &Transcoding,
) -> Result<Stream, super::Error> {
    let path = transcoding.url();

    let res = match agent.get(&path).query("client_id", client_id).call() {
        Ok(res) => res,
        Err(err) => return Err(crate::Error::Ureq(Box::new(err))),
    };

    match res.into_json::<Stream>() {
        Ok(stream) => Ok(stream),
        Err(err) => Err(crate::Error::StdIo(err)),
    }
}

pub(crate) fn get_stream_bytes(agent: &Agent, stream: &Stream) -> Result<Vec<u8>, super::Error> {
    let path = stream.url();
    get_bytes(agent, path.as_str())
}

pub(crate) fn get_bytes(agent: &Agent, url: &str) -> Result<Vec<u8>, super::Error> {
    let path = url;

    let res = match agent.get(&path).call() {
        Ok(res) => res,
        Err(err) => return Err(crate::Error::Ureq(Box::new(err))),
    };

    let mut bytes: Vec<u8> = Vec::new();
    if res.into_reader().read_to_end(&mut bytes).is_ok() {
        return Ok(bytes);
    }

    Err(crate::Error::InvalidData(String::from(path)))
}

pub(crate) fn get_search(
    agent: &Agent,
    client_id: &str,
    query: &str,
    limit: i64,
    offset: i64,
) -> Result<Collection, super::Error> {
    let path = format!("{SOUNDCLOUD_API_V2}{SEARCH}");

    let res = match agent
        .get(&path)
        .query("client_id", client_id)
        .query("q", query)
        .query("limit", &limit.to_string())
        .query("offset", &offset.to_string())
        .call()
    {
        Ok(res) => res,
        Err(err) => return Err(crate::Error::Ureq(Box::new(err))),
    };

    match res.into_json::<Collection>() {
        Ok(search_results) => Ok(search_results),
        Err(err) => Err(crate::Error::StdIo(err)),
    }
}

pub(crate) fn get_client_id(agent: &Agent) -> Result<String, super::Error> {
    let res = match agent.get(SOUNDCLOUD).call() {
        Ok(res) => res,
        Err(err) => return Err(crate::Error::Ureq(Box::new(err))),
    };

    let body = match res.into_string() {
        Ok(body) => body,
        Err(err) => return Err(crate::Error::StdIo(err)),
    };

    let re = match Regex::new(
        r"(https)://[\w-]+(\.[\w-]+)+([\w.,@?^=%&amp;:/~+#-]*[\w@?^=%&amp;/~+#-])?",
    ) {
        Ok(re) => re,
        Err(err) => return Err(crate::Error::Regex(err)),
    };

    for capture in re.captures_iter(&body) {
        let url = &capture[0];
        if url.contains("https://a-v2.sndcdn.com/assets/") && url.contains(".js") {
            let Ok(res) = agent.get(url).call() else {
                continue;
            };
            let Ok(body) = res.into_string() else {
                continue;
            };
            let Ok(re) = Regex::new(r#",client_id:"(.*?)""#) else {
                continue;
            };
            let Some(captures) = re.captures(&body) else {
                continue;
            };
            return Ok(String::from(&captures[1]));
        }
    }

    Err(crate::Error::Unknown)
}

#[cfg(test)]
mod tests {
    use core::panic;

    use crate::client::AGENT;

    use super::get_client_id;

    #[test]
    fn test_client_id() {
        let agent = &AGENT.clone();

        match get_client_id(agent) {
            Ok(client_id) => println!("{client_id:?}"),
            Err(err) => panic!("{err:?}"),
        };
    }
}
