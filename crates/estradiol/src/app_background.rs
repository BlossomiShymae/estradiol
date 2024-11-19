use std::{
    io::Cursor,
    num::NonZeroUsize,
    sync::mpsc::{Receiver, Sender},
};

use discord_rich_presence::{activity, DiscordIpc, DiscordIpcClient};
use estradiol_soundcloud::{models::collections::Collection, Client};
use lru::LruCache;
use rodio::{Decoder, OutputStream, Sink};

use crate::app::UiEvent;

#[derive(Debug)]
pub enum BackgroundEvent {
    SearchComplete(Collection),
}

pub fn run_background(
    background_event_rx: Receiver<UiEvent>,
    ui_event_tx: Sender<BackgroundEvent>,
) -> impl Fn() {
    move || {
        let client = Client::new();
        let mut track_cache: LruCache<i64, Vec<u8>> = LruCache::new(NonZeroUsize::new(4).unwrap());
        let (_stream, stream_handle) = match OutputStream::try_default() {
            Ok(stream) => stream,
            Err(err) => {
                eprintln!("{err:?}");
                return;
            }
        };
        let sink = match Sink::try_new(&stream_handle) {
            Ok(sink) => sink,
            Err(err) => {
                eprintln!("{err:?}");
                return;
            }
        };

        while let Ok(event) = background_event_rx.recv() {
            match event {
                UiEvent::SearchSubmit(query) => {
                    println!("Searching for {query}");
                    if let Ok(results) = client.search(query.as_str(), 50, 0) {
                        println!("Obtained {} results", results.collection().len());
                        let _ = ui_event_tx.send(BackgroundEvent::SearchComplete(results));
                    }
                }
                UiEvent::PlayTrack(id) => {
                    println!("Request to play track {id}");
                    if let Some(track_bytes) = track_cache.get(&id) {
                        play_track(track_bytes.clone(), &sink);
                    } else {
                        let Ok(track) = client.track(id) else {
                            return;
                        };
                        let Some(media) = track.media() else {
                            return;
                        };
                        let Some(transcoding) = media.progressive() else {
                            return;
                        };
                        let Ok(stream) = client.stream(&transcoding) else {
                            return;
                        };
                        let Ok(track_bytes) = client.bytes(&stream.url()) else {
                            return;
                        };

                        track_cache.put(id, track_bytes.clone());
                        play_track(track_bytes, &sink);
                    }
                }
            }
        }
    }
}

fn play_track(track_bytes: Vec<u8>, sink: &Sink) {
    let Ok(source) = Decoder::new(Cursor::new(track_bytes)) else {
        return;
    };

    sink.clear();
    sink.append(source);
    sink.play();
    println!("Playing track");
}
