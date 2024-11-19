use std::sync::mpsc::channel;

use app::{App, UiEvent};
use app_background::BackgroundEvent;
use egui::Theme;

pub mod anchor_state;
pub mod app;
mod app_background;
pub mod apps;
pub use app_background::run_background;
use utils::Channel;
pub mod utils;

fn main() -> eframe::Result {
    let (background_event_tx, background_event_rx) = channel::<UiEvent>();
    let (ui_event_tx, ui_event_rx) = channel::<BackgroundEvent>();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),

        ..Default::default()
    };

    std::thread::spawn(run_background(background_event_rx, ui_event_tx));

    eframe::run_native(
        "Estradiol",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_theme(Theme::Dark);
            Ok(Box::new(App::new(Channel::new(
                background_event_tx,
                ui_event_rx,
            ))))
        }),
    )
}
