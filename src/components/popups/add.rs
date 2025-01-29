use super::{center_area, Popup};
use crate::types::PopupMessages;
use crossterm::event::{self, KeyCode, KeyEventKind};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Text};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Frame;
use std::sync::mpsc;
use tui_textarea::TextArea;

#[derive(PartialEq)]
enum Section {
    EmailField,
    PasswordField,
    AddButton,
    CancelButton,
}

pub struct Add<'ta> {
    screen_sender: mpsc::Sender<PopupMessages>,
    current_section: Section,
    is_valid: Option<bool>,
    pub email: TextArea<'ta>,
    pub password: TextArea<'ta>,
}

impl<'ta> Add<'ta> {
    pub fn new(screen_sender: mpsc::Sender<PopupMessages>) -> Self {
        let mut email: TextArea = TextArea::default();
        let mut password: TextArea = TextArea::default();
        email.set_placeholder_text("example@gmail.com");
        password.set_mask_char('🫣');
        Self {
            current_section: Section::EmailField,
            is_valid: None,
            email,
            password,
            screen_sender,
        }
    }

    fn get_email_block(&self) -> Block<'ta> {
        Block::default().title("Email").borders(Borders::ALL).style(
            if self.current_section == Section::EmailField {
                selected_style()
            } else {
                unselected_style()
            },
        )
    }

    fn get_password_block(&self) -> Block<'ta> {
        Block::default()
            .title("Password")
            .borders(Borders::ALL)
            .style(if self.current_section == Section::PasswordField {
                selected_style()
            } else {
                unselected_style()
            })
    }

    fn get_add_block(&self) -> Block {
        Block::default().borders(Borders::ALL).style(
            if self.current_section == Section::AddButton {
                selected_style()
            } else {
                unselected_style()
            },
        )
    }

    fn get_cancel_block(&self) -> Block {
        Block::default().borders(Borders::ALL).style(
            if self.current_section == Section::CancelButton {
                selected_style()
            } else {
                unselected_style()
            },
        )
    }

    pub fn get_fields(&mut self) -> Vec<Paragraph> {
        self.email.set_block(self.get_email_block());
        self.password.set_block(self.get_password_block());
        let instruction = Text::from(vec![
            Line::from("Use <Tab> to switch between selected fields"),
            Line::from("Please use google's app password for gmail"),
        ]);
        let instruction = Paragraph::new(instruction).centered();
        let add_button = Paragraph::new("Add").block(self.get_add_block()).centered();
        let cancel_button = Paragraph::new("Cancel")
            .block(self.get_cancel_block())
            .centered();
        vec![cancel_button, add_button, instruction]
    }

    // Given the area this function will return 5 Rects for,
    // EmailField, PasswordField, Instruction, Ok button, Cancel Button.
    pub fn get_area_chunks(area: Rect) -> Vec<Rect> {
        let mut popup_area = Vec::new();
        let layout = Layout::vertical([
            Constraint::Max(3),
            Constraint::Max(3),
            Constraint::Fill(1),
            Constraint::Max(3),
        ])
        .spacing(1)
        .margin(2);
        let chunks: [Rect; 4] = layout.areas(area);
        popup_area.push(chunks[0]);
        popup_area.push(chunks[1]);
        popup_area.push(chunks[2]);
        let layout = Layout::horizontal([Constraint::Fill(1), Constraint::Fill(1)]).spacing(2);
        let chunks: [Rect; 2] = layout.areas(chunks[3]);
        popup_area.push(chunks[0]);
        popup_area.push(chunks[1]);
        popup_area
    }

    fn check_valid(&mut self) {
        let email = self.email.lines()[0].clone();
        let password = self.password.lines()[0].clone();
        if email.ends_with("@gmail.com") && !password.is_empty() {
            self.is_valid = Some(true);
            return;
        }
        self.is_valid = Some(false);
    }
}

impl<'ta> Popup for Add<'ta> {
    fn show(&mut self, frame: &mut Frame) {
        // Add code which will render the add popup inscreen.
        // NOTE: Get center area of screen (function in popups/mod.rs), I think this could be done in
        // main loop just before calling `show` function and passing the area.
        // Get area chunks where all the fileds will be rendered.
        // Get all the fields.
        // Render all the fields in there respective chunk.
        let block = Block::new()
            .borders(Borders::ALL)
            .title("Add Email and Password")
            .style(
                Style::default()
                    .bg(Color::Black)
                    .fg(if self.is_valid == Some(false) {
                        Color::Red
                    } else {
                        Color::White
                    }),
            );
        let mut fields = self.get_fields();
        let centered_area =
            center_area(frame.area(), Constraint::Length(65), Constraint::Length(20));
        let popup_areas = Add::get_area_chunks(centered_area);
        frame.render_widget(block, centered_area);
        frame.render_widget(fields.pop().unwrap(), popup_areas[2]);
        frame.render_widget(fields.pop().unwrap(), popup_areas[3]);
        frame.render_widget(fields.pop().unwrap(), popup_areas[4]);
        frame.render_widget(&self.email, popup_areas[0]);
        frame.render_widget(&self.password, popup_areas[1]);
        let _ = self.screen_sender.send(PopupMessages::ShowAddPopup);
    }

    fn hide(self) {
        let _ = self.screen_sender.send(PopupMessages::HideAddPopup);
    }

    fn handle_input(&mut self) {
        if let event::Event::Key(key) = event::read().unwrap() {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Tab => match self.current_section {
                        Section::EmailField => self.current_section = Section::PasswordField,
                        Section::PasswordField => self.current_section = Section::AddButton,
                        Section::AddButton => self.current_section = Section::CancelButton,
                        Section::CancelButton => self.current_section = Section::EmailField,
                    },
                    KeyCode::Enter => match self.current_section {
                        Section::AddButton => {
                            self.check_valid();
                            if self.is_valid == Some(true) {
                                self.screen_sender.send(PopupMessages::AddCred);
                                self.screen_sender.send(PopupMessages::HideAddPopup);
                            }
                        }
                        Section::CancelButton => {
                            self.screen_sender.send(PopupMessages::HideAddPopup);
                        }
                        Section::EmailField => self.current_section = Section::PasswordField,
                        Section::PasswordField => self.current_section = Section::AddButton,
                    },
                    KeyCode::Char(ch) => match self.current_section {
                        Section::EmailField => {
                            self.email.insert_char(ch);
                        }
                        Section::PasswordField => {
                            self.password.insert_char(ch);
                        }
                        _ => {}
                    },
                    KeyCode::Backspace => match self.current_section {
                        Section::EmailField => {
                            self.email.delete_char();
                        }
                        Section::PasswordField => {
                            self.password.delete_char();
                        }
                        _ => {}
                    },
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
