use crate::{
    anchor_state::{Anchor, AnchorState},
    app_background::BackgroundEvent,
    utils::Channel,
};

#[derive(Debug)]
pub enum UiEvent {
    SearchSubmit(String),
    PlayTrack(i64),
}

#[derive(Debug)]
pub struct App {
    anchor_state: AnchorState,
    channel: Channel,
}

impl App {
    pub fn new(channel: Channel) -> Self {
        Self {
            anchor_state: AnchorState::new(channel.clone()),
            channel: channel.clone(),
        }
    }

    fn apps_iter_mut(&mut self) -> impl Iterator<Item = (&str, Anchor, &mut dyn eframe::App)> {
        let vec = vec![(
            "Search",
            Anchor::Search,
            &mut self.anchor_state.search as &mut dyn eframe::App,
        )];

        vec.into_iter()
    }

    fn bar_contents(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut selected_anchor = self.anchor_state.selected_anchor;
        ui.horizontal(|ui| {
            for (name, anchor, _app) in self.apps_iter_mut() {
                if ui
                    .selectable_label(selected_anchor == anchor, name)
                    .clicked()
                {
                    selected_anchor = anchor;
                }
            }
        });
        self.anchor_state.selected_anchor = selected_anchor;
    }

    fn show_selected_app(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let selected_anchor = self.anchor_state.selected_anchor;
        for (_name, anchor, app) in self.apps_iter_mut() {
            if anchor == selected_anchor || ctx.memory(egui::Memory::everything_is_visible) {
                app.update(ctx, frame);
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);

        for event in self.channel.rx().try_iter() {
            match event {
                BackgroundEvent::SearchComplete(results) => {
                    self.anchor_state.search.set_results(Some(results));
                }
                _ => (),
            }
        }

        egui::TopBottomPanel::top("app_top_bar")
            .frame(egui::Frame::none().inner_margin(4.0))
            .show(ctx, |ui| {
                ui.visuals_mut().button_frame = false;
                self.bar_contents(ui, frame);
            });

        egui::CentralPanel::default().show(ctx, |_ui| {
            self.show_selected_app(ctx, frame);
        });
    }
}
