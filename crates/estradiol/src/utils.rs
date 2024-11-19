use std::sync::{
    mpsc::{Receiver, Sender},
    Arc,
};

use crate::{app::UiEvent, app_background::BackgroundEvent};

#[derive(Debug, Clone)]
pub struct Channel {
    tx: Arc<Sender<UiEvent>>,
    rx: Arc<Receiver<BackgroundEvent>>,
}

impl Channel {
    pub fn new(tx: Sender<UiEvent>, rx: Receiver<BackgroundEvent>) -> Self {
        Self {
            tx: Arc::new(tx),
            rx: Arc::new(rx),
        }
    }

    pub fn tx(&self) -> Arc<Sender<UiEvent>> {
        Arc::clone(&self.tx)
    }

    pub fn rx(&self) -> Arc<Receiver<BackgroundEvent>> {
        Arc::clone(&self.rx)
    }
}
