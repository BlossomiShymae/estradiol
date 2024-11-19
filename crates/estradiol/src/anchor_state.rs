use crate::{apps::search::SearchApp, utils::Channel};

#[derive(Debug, PartialEq, Copy, Clone, Default)]
pub enum Anchor {
    #[default]
    Search,
}

impl std::fmt::Display for Anchor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl From<Anchor> for egui::WidgetText {
    fn from(value: Anchor) -> Self {
        Self::RichText(egui::RichText::new(value.to_string()))
    }
}

#[derive(Debug)]
pub struct AnchorState {
    pub search: SearchApp,
    pub selected_anchor: Anchor,
}

impl AnchorState {
    pub fn new(channel: Channel) -> Self {
        Self {
            selected_anchor: Anchor::Search,
            search: SearchApp::new(channel.clone()),
        }
    }
}
