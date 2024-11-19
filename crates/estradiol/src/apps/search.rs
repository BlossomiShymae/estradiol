use egui_extras::{Size, StripBuilder};
use estradiol_soundcloud::models::{
    collections::Collection,
    resources::{Resource, ResourceKind},
};

use crate::{app::UiEvent, utils::Channel};

#[derive(Debug)]
pub struct SearchApp {
    search: String,
    results: Option<Collection>,
    selected_resource: Option<Resource>,
    channel: Channel,
}

impl SearchApp {
    pub fn new(channel: Channel) -> Self {
        Self {
            search: String::from(""),
            results: None,
            selected_resource: None,
            channel,
        }
    }

    pub fn set_results(&mut self, results: Option<Collection>) {
        self.results = results;
    }
}

impl eframe::App for SearchApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.search);
                if ui.button("search").clicked() {
                    let _ = self
                        .channel
                        .tx()
                        .send(UiEvent::SearchSubmit(self.search.clone()));
                }
            });
            StripBuilder::new(ui)
                .size(Size::remainder())
                .size(Size::initial(160.0))
                .vertical(|mut strip| {
                    strip.cell(|ui| {
                        egui::ScrollArea::vertical().animated(true).show(ui, |ui| {
                            let Some(results) = &self.results else {
                                return;
                            };
                            let mut selected_resource = self.selected_resource.clone();
                            for resource in results.collection().into_iter() {
                                let ResourceKind::Track = resource.kind() else {
                                    continue;
                                };
                                let selected = {
                                    if let Some(selected_resource) = selected_resource.clone() {
                                        selected_resource.id() == resource.id()
                                    } else {
                                        false
                                    }
                                };
                                if ui
                                    .selectable_label(
                                        selected,
                                        resource.title().unwrap_or_default(),
                                    )
                                    .clicked()
                                {
                                    selected_resource = Some(resource);
                                }
                            }
                            self.selected_resource = selected_resource;
                        });
                    });
                    strip.cell(|ui| {
                        ui.separator();
                        StripBuilder::new(ui)
                            .size(Size::initial(160.0))
                            .size(Size::remainder())
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    let Some(selected_resource) = self.selected_resource.clone()
                                    else {
                                        return;
                                    };
                                    let Some(artwork_url) = selected_resource.artwork_url() else {
                                        return;
                                    };
                                    ui.add(
                                        egui::Image::from_uri(artwork_url)
                                            .max_size(egui::Vec2 { x: 160.0, y: 160.0 }),
                                    );
                                });
                                strip.cell(|ui| {
                                    let Some(selected_resource) = self.selected_resource.clone()
                                    else {
                                        return;
                                    };
                                    ui.label(selected_resource.title().unwrap_or_default());
                                    if let Some(user) = selected_resource.user() {
                                        ui.label(user.username().unwrap_or_default());
                                    }
                                    if ui.button("play").clicked() {
                                        let _ = self
                                            .channel
                                            .tx()
                                            .send(UiEvent::PlayTrack(selected_resource.id()));
                                    }
                                });
                            });
                    });
                });
        });
    }
}
