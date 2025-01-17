use super::{center_area, Popup};
use std::sync::mpsc;

enum Section {
    No,
    Yes,
}

pub struct Quit {
    screen_sender: mpsc::Sender<PopupMessages>,
    current_section: Section,
}

impl Quit {
    fn new(screen_sender: mpsc::Sender<PopupMessages>) -> Self {
        Self {
            current_section: Section::No,
            screen_sender,
        }
    }
}

impl Popup for Quit {
    fn show(&mut self, frame: &mut Frame) {}
    fn hide(&mut self) {}
    fn handle_input(&mut self) {}
}
