use std::rc::Rc;
use std::sync::mpsc::Sender;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{
    layout::{Layout, Alignment, Rect},
    prelude::{Direction, Constraint},
    style::{Style, Color, Stylize},
    Frame
};
use crate::windows::PopupStatus;

pub enum QuitStatus {
    Show,
    Yes,
    No,
}

pub struct Quit {
    ch_popup_sender: Sender<PopupStatus>,
    pub which: u8,
}

impl Quit {
    pub fn new(ch_popup_sender: Sender<PopupStatus>) -> Self {
        Self { ch_popup_sender, which: 0 }
    }

    pub fn draw(&self, frame: &mut Frame) {
        let popup_area = get_popup_area(frame.area());
        let popup_chunks = get_chunks(popup_area.clone());
        let button_chunks = get_button_chunks(popup_chunks[1].clone());
        let block = Block::default()
            .title("Quit")
            .style(Style::default().bg(Color::Black))
            .borders(Borders::ALL);
        let paragraph = Paragraph::new("Are you sure you want to quit?")
            .alignment(Alignment::Center)
            .wrap(Wrap{trim: false});
        let ok_block = Block::default()
            .style(Style::default().bg(if self.which == 0 {Color::Cyan} else {Color::Black}));
        let cancel_block = Block::default()
            .style(Style::default().bg(if self.which == 1 {Color::Cyan} else {Color::Black}));
        let ok_button = Paragraph::new("Yes")
            .block(ok_block)
            .white()
            .alignment(Alignment::Center);
        let cancel_button = Paragraph::new("No")
            .block(cancel_block)
            .white()
            .alignment(Alignment::Center);
        frame.render_widget(block, popup_area);
        frame.render_widget(paragraph, popup_chunks[0]);
        frame.render_widget(ok_button, button_chunks[0]);
        frame.render_widget(cancel_button, button_chunks[1]);
    }
}

fn get_popup_area(r: Rect) -> Rect {
    let [_, ver_area, _] = Layout::vertical([
            Constraint::Percentage((100 - 30)/2),
            Constraint::Fill(1),
            Constraint::Percentage((100 - 30)/2),
        ])
        .areas(r);

    let [_, box_area, _] = Layout::horizontal([
            Constraint::Percentage((100 - 30)/2),
            Constraint::Fill(1),
            Constraint::Percentage((100 - 30)/2),
        ])
        .areas(ver_area);

    box_area
}

// fn get_board(frame: &mut Frame) -> Option<(String, String)> {
fn get_chunks(r: Rect) -> Rc<[Rect]> {
    let popup_chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Fill(3),
            Constraint::Min(1),
        ])
        .split(r);
    popup_chunks
}

fn get_button_chunks(r: Rect) -> Rc<[Rect]> {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(r);
    chunks
}
