use crate::components::popups::{center_area, Add, Popup};
use crate::types::PopupMessages;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::layout::Constraint;
use ratatui::widgets::Paragraph;
use ratatui::DefaultTerminal;
use std::io;
use std::sync::mpsc::channel;

enum CurrentPopup<'ta> {
    AddPopup(Add<'ta>),
}

pub fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let (sender, reciver) = channel();
    let mut current_popup: Option<CurrentPopup> = None;
    loop {
        terminal.draw(|frame| {
            if let Some(popup) = &mut current_popup {
                match popup {
                    CurrentPopup::AddPopup(add_popup) => {
                        // let mut fields = add_popup.get_fields();
                        // let center = center_area(
                        // frame.area(),
                        // Constraint::Length(65),
                        // Constraint::Length(20),
                        // );
                        // let popup_areas = Add::get_area_chunks(center);
                        // frame.render_widget(fields[0].clone(), popup_areas[2]);
                        // frame.render_widget(fields[1].clone(), popup_areas[3]);
                        // frame.render_widget(fields[2].clone(), popup_areas[4]);
                        // frame.render_widget(&add_popup.email, popup_areas[0]);
                        // frame.render_widget(&add_popup.password, popup_areas[1]);
                        add_popup.show(frame);
                    }
                }
            }
            let p = Paragraph::new("Welcome to chitthi").centered();
            frame.render_widget(p, frame.area());
        })?;
        if let Some(popup) = &mut current_popup {
            match popup {
                CurrentPopup::AddPopup(add_popup) => {
                    add_popup.handle_input();
                }
                _ => {}
            }
        } else {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('a') => {
                            let popup = Add::new(sender.clone());
                            current_popup = Some(CurrentPopup::AddPopup(popup))
                        }
                        KeyCode::Esc => return Ok(()),
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
                _ => {}
            }
        }
    }
}
