use crate::windows::PopupStatus;
use ratatui::{
    layout::{Alignment, Layout, Rect},
    prelude::{Constraint, Direction},
    style::{Color, Style, Stylize},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use std::io;
use std::rc::Rc;
use std::sync::mpsc;
use tui_textarea::{Input, Key, TextArea};

pub enum AddPopupStatus {
    Show,
    Save,
    Exit,
}

#[derive(PartialEq, Debug)]
pub enum WhichSection {
    EmailBox,
    PasswordBox,
    OkBox,
    CancelBox,
}

pub struct AddCredPopup<'text_area> {
    pub which: WhichSection,
    pub email: TextArea<'text_area>,
    pub password: TextArea<'text_area>,
    ch_popup_sender: mpsc::Sender<PopupStatus>,
}

impl<'text_area> AddCredPopup<'text_area> {
    pub fn new(ch: mpsc::Sender<PopupStatus>) -> Self {
        Self {
            which: WhichSection::EmailBox,
            email: TextArea::default(),
            password: TextArea::default(),
            ch_popup_sender: ch,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        self.draw_box(frame);
        self.handle_input().expect("unable to handle input");
    }

    pub fn draw_box(&mut self, frame: &mut Frame) {
        // create layout.
        let popup_area = get_popup_area(frame.area());
        let popup_chunks = get_chunks(popup_area.clone());
        let button_chunks = get_button_chunks(popup_chunks[3].clone());
        // create blocks.
        let add_cred_block = Block::default()
            .title("Add credentials")
            .title_alignment(Alignment::Center)
            .borders(Borders::TOP | Borders::BOTTOM)
            .border_style(Style::default().fg(Color::Green))
            .on_black();
        let email_block = Block::default()
            .title("Email")
            .style(
                Style::default().bg(if self.which == WhichSection::EmailBox {
                    Color::Cyan
                } else {
                    Color::Black
                }),
            )
            .borders(Borders::ALL);
        let password_block = Block::default()
            .title("password")
            .style(
                Style::default().bg(if self.which == WhichSection::PasswordBox {
                    Color::Cyan
                } else {
                    Color::Black
                }),
            )
            .borders(Borders::ALL);
        let ok_block =
            Block::default().style(Style::default().bg(if self.which == WhichSection::OkBox {
                Color::Cyan
            } else {
                Color::Black
            }));
        let cancel_block = Block::default().style(Style::default().bg(
            if self.which == WhichSection::CancelBox {
                Color::Cyan
            } else {
                Color::Black
            },
        ));
        // make paragraph.
        let hint = if self.email.lines()[0].is_empty() {
            Paragraph::new(
                "We recommendto use app password. \nUse Tab to switch bwtween active blocks",
            )
            .white()
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center)
        } else {
            match self.is_valid() {
                Ok(_) => Paragraph::new(
                    "We recommendto use app password. \nUse Tab to switch bwtween active blocks",
                )
                .white()
                .wrap(Wrap { trim: false })
                .alignment(Alignment::Center),
                Err(msg) => Paragraph::new(msg)
                    .red()
                    .wrap(Wrap { trim: false })
                    .alignment(Alignment::Center),
            }
        };
        self.email.set_block(email_block);
        self.password.set_block(password_block);
        let ok_button = Paragraph::new("Add")
            .block(ok_block)
            .white()
            .alignment(Alignment::Center);
        let cancel_button = Paragraph::new("Cancel")
            .block(cancel_block)
            .white()
            .alignment(Alignment::Center);
        // create textarea.
        self.email.set_placeholder_text("example@gmail.com");
        self.email.set_cursor_line_style(Style::default());
        // adding error message.
        self.password.set_cursor_line_style(Style::default());
        self.password.set_mask_char('\u{2022}');
        // render widgets.
        frame.render_widget(add_cred_block, popup_area);
        frame.render_widget(&self.email, popup_chunks[0]);
        frame.render_widget(&self.password, popup_chunks[1]);
        frame.render_widget(hint, popup_chunks[2]);
        frame.render_widget(ok_button, button_chunks[0]);
        frame.render_widget(cancel_button, button_chunks[1]);
    }

    fn is_valid(&mut self) -> Result<(), String> {
        let is_email_val = self.email.lines()[0].contains("@gmail.com");
        let is_password_val = !self.password.lines()[0].is_empty();
        let mut error_msg = String::new();
        if !is_email_val {
            error_msg = "Please enter a valid gmail email".to_string();
        } else if !is_password_val {
            error_msg = "Please enter Password.".to_string();
        }
        if error_msg.is_empty() {
            return Ok(());
        }
        Err(error_msg)
    }

    fn handle_input(&mut self) -> io::Result<()> {
        match crossterm::event::read()?.into() {
            Input { key: Key::Tab, .. } => {
                self.which = match self.which {
                    WhichSection::EmailBox => WhichSection::PasswordBox,
                    WhichSection::PasswordBox => WhichSection::OkBox,
                    WhichSection::OkBox => WhichSection::CancelBox,
                    WhichSection::CancelBox => WhichSection::EmailBox,
                }
            }
            Input {
                key: Key::Enter, ..
            } => {
                if self.which == WhichSection::OkBox {
                    if let Ok(_) = self.is_valid() {
                        self.ch_popup_sender
                            .send(PopupStatus::Add(AddPopupStatus::Save))
                            .unwrap();
                        self.ch_popup_sender
                            .send(PopupStatus::Add(AddPopupStatus::Exit))
                            .unwrap();
                    }
                } else if self.which == WhichSection::CancelBox {
                    self.ch_popup_sender
                        .send(PopupStatus::Add(AddPopupStatus::Exit))
                        .unwrap();
                }
            }
            input => {
                if self.which == WhichSection::EmailBox {
                    self.email.input(input);
                } else if self.which == WhichSection::PasswordBox {
                    self.password.input(input);
                };
            }
        }
        Ok(())
    }
}

fn get_popup_area(r: Rect) -> Rect {
    let [_, ver_area, _] = Layout::vertical([
        Constraint::Percentage((100 - 39) / 2),
        Constraint::Percentage(39),
        Constraint::Percentage((100 - 39) / 2),
    ])
    .areas(r);

    let [_, box_area, _] = Layout::horizontal([
        Constraint::Percentage((100 - 43) / 2),
        Constraint::Percentage(43),
        Constraint::Percentage((100 - 43) / 2),
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
            Constraint::Max(3),
            Constraint::Max(3),
            Constraint::Fill(1),
            Constraint::Max(1),
        ])
        .split(r);
    popup_chunks
}

fn get_button_chunks(r: Rect) -> Rc<[Rect]> {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Fill(1), Constraint::Fill(1)])
        .split(r);
    chunks
}
