use super::{center_area, Popup};
use crate::types::PopupMessages;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use std::sync::mpsc;

#[derive(PartialEq)]
enum Section {
    No,
    Yes,
}

pub struct Quit {
    screen_sender: mpsc::Sender<PopupMessages>,
    current_section: Section,
}

impl Quit {
    pub fn new(screen_sender: mpsc::Sender<PopupMessages>) -> Self {
        Self {
            current_section: Section::No,
            screen_sender,
        }
    }

    fn get_no_block(&self) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .style(if self.current_section == Section::No {
                selected_style()
            } else {
                unselected_style()
            })
    }

    fn get_yes_block(&self) -> Block {
        Block::default()
            .borders(Borders::ALL)
            .style(if self.current_section == Section::Yes {
                selected_style()
            } else {
                unselected_style()
            })
    }

    fn get_fields(&self) -> [Paragraph; 3] {
        let instruction = Paragraph::new("Do you want to quit").alignment(Alignment::Center);
        let yes_button = Paragraph::new("Yes")
            .block(self.get_yes_block())
            .alignment(Alignment::Center);
        let no_button = Paragraph::new("No")
            .block(self.get_no_block())
            .alignment(Alignment::Center);
        [instruction, yes_button, no_button]
    }

    fn get_area_chunks(area: Rect) -> Vec<Rect> {
        let mut popup_areas = Vec::new();
        let layout = Layout::vertical([Constraint::Fill(1), Constraint::Max(3)])
            .spacing(1)
            .margin(2);
        let chunks: [Rect; 2] = layout.areas(area);
        popup_areas.push(chunks[0]);
        let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).spacing(2);
        let chunks: [Rect; 2] = layout.areas(chunks[1]);
        popup_areas.push(chunks[0]);
        popup_areas.push(chunks[1]);
        popup_areas
    }
}

impl Popup for Quit {
    fn show(&mut self, frame: &mut Frame) {
        let block = Block::new()
            .borders(Borders::ALL)
            .title("Quit Popup")
            .style(Style::default().bg(Color::Black).fg(Color::White));
        let [instruction, yes_button, no_button] = self.get_fields();
        let centered_area =
            center_area(frame.area(), Constraint::Length(40), Constraint::Length(10));
        let popup_areas = Quit::get_area_chunks(centered_area);
        frame.render_widget(block, centered_area);
        frame.render_widget(instruction, popup_areas[0]);
        frame.render_widget(yes_button, popup_areas[1]);
        frame.render_widget(no_button, popup_areas[2]);
        let _ = self.screen_sender.send(PopupMessages::ShowQuitPopup);
    }

    fn hide(self) {
        self.screen_sender.send(PopupMessages::HideQuitPopup);
    }
    fn handle_input(&mut self) {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Tab => match self.current_section {
                        Section::No => self.current_section = Section::Yes,
                        Section::Yes => self.current_section = Section::No,
                    },
                    KeyCode::Enter => match self.current_section {
                        Section::Yes => self.screen_sender.send(PopupMessages::QuitApp),
                        Section::No => self.screen_sender.send(PopupMessages::HideQuitPopup),
                    }
                    .unwrap(),
                    _ => {}
                }
            }
        }
    }
}

fn selected_style() -> Style {
    Style::default().fg(Color::White).bg(Color::Cyan)
}

fn unselected_style() -> Style {
    Style::default().fg(Color::White).bg(Color::Black)
}
