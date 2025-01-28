use crate::components::popups::{center_area, Add, Popup, Quit};
use crate::types::PopupMessages;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::layout::{Alignment, Constraint, Direction, Layout};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{symbols, DefaultTerminal};
use std::io;
use std::sync::mpsc::channel;

pub enum CurrentPopup<'ta> {
    AddPopup(Add<'ta>),
    QuitPopup(Quit),
}

pub fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let instruction = Line::from(vec![
        " Add new account: ".into(),
        "<A>".green().bold(),
        " View added accounts: ".into(),
        "<V>".green().bold(),
        " Quit: ".into(),
        "<Esc> ".red().bold(),
    ]);
    let bottom_block = Block::bordered()
        .title_bottom(instruction.white().centered())
        .border_set(symbols::border::DOUBLE);
    let greeting = Paragraph::new("Hello! welcome to chitthi, your in-terminal mail manager")
        .block(bottom_block)
        .cyan()
        .on_black()
        .alignment(Alignment::Center);
    let mut current_popup: Option<CurrentPopup> = None;
    let (sender, reciver) = channel();
    loop {
        terminal.draw(|frame| {
            let chunk = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Fill(1)])
                .margin(5)
                .split(frame.area());
            frame.render_widget(&greeting, chunk[0]);
            if let Some(popup) = &mut current_popup {
                match popup {
                    CurrentPopup::AddPopup(add_popup) => add_popup.show(frame),
                    CurrentPopup::QuitPopup(quit_popup) => quit_popup.show(frame),
                    _ => {}
                }
            }
        })?;
        if let Some(popup) = &mut current_popup {
            match popup {
                CurrentPopup::AddPopup(add_popup) => add_popup.handle_input(),
                CurrentPopup::QuitPopup(quit_popup) => quit_popup.handle_input(),
            }
        } else {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('a') => {
                            let popup = Add::new(sender.clone());
                            current_popup = Some(CurrentPopup::AddPopup(popup))
                        }
                        KeyCode::Esc => {
                            let popup = Quit::new(sender.clone());
                            current_popup = Some(CurrentPopup::QuitPopup(popup))
                        }
                        _ => {}
                    }
                }
            }
        }
        if let Ok(message) = reciver.try_recv() {
            match message {
                PopupMessages::ShowAddPopup => {}
                PopupMessages::HideAddPopup => {
                    current_popup = None;
                }
                PopupMessages::HideQuitPopup => {
                    current_popup = None;
                }
                PopupMessages::QuitApp => {
                    return Ok(());
                }
                _ => {}
            }
        }
    }
}
