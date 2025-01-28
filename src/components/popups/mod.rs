pub use add::Add;
pub use quit::Quit;
use ratatui::layout::{Constraint, Flex, Layout, Rect};
use ratatui::Frame;
mod add;
mod quit;

pub trait Popup {
    fn show(&mut self, frame: &mut Frame);
    fn hide(&mut self);
    fn handle_input(&mut self);
}

pub fn center_area(area: Rect, horizontal: Constraint, vertical: Constraint) -> Rect {
    let [area] = Layout::horizontal([horizontal])
        .flex(Flex::Center)
        .areas(area);
    let [area] = Layout::vertical([vertical]).flex(Flex::Center).areas(area);
    area
}
