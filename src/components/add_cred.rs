use std::rc::{Rc};
use std::io;
use std::sync::mpsc;
use ratatui::{
    style::{Style, Color, Stylize},
    prelude::{Direction, Constraint},
    layout::{Rect, Layout, Alignment},
    widgets::{Borders, Block, Paragraph, Wrap},
    Frame
};
use tui_textarea::{Input, Key, TextArea};
use crate::windows::PopupStatus;

pub enum AddPopupStatus {
    Show,
    Save,
    Exit,
}

pub struct AddCredPopup<'text_area> {
    pub which: usize,
    pub email: TextArea<'text_area>,
    pub password: TextArea<'text_area>,
    ch_popup_sender: mpsc::Sender<PopupStatus>,
}

impl<'text_area> AddCredPopup<'text_area> {
    pub fn new(ch: mpsc::Sender<PopupStatus>) -> Self {
        Self {
            which: 0,
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
            .style(Style::default().bg(if self.which == 0 {Color::Cyan} else {Color::Black}))
            .borders(Borders::ALL);
        let password_block = Block::default()
            .title("password")
            .style(Style::default().bg(if self.which == 1 {Color::Cyan} else {Color::Black}))
            .borders(Borders::ALL);
        let ok_block = Block::default()
            .style(Style::default().bg(if self.which == 2 {Color::Cyan} else {Color::Black}));
        let cancel_block = Block::default()
            .style(Style::default().bg(if self.which == 3 {Color::Cyan} else {Color::Black}));
        // make paragraph.
        let hint = Paragraph::new("We recommend to use 'app' password.\nUse Tab to switch between active blocks")
            .white()
            .wrap(Wrap { trim: false })
            .alignment(Alignment::Center);
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
        if self.which > 1 {
            self.email.set_block(email_block);
            self.password.set_block(password_block);
        } else {
            let _ = self.is_valid();
        }
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

    fn is_valid(&mut self) -> bool {
        let mut is_email_val = false;
        if (self.email.lines()[0]).contains("@gmail.com") {
            is_email_val = true;
            self.email.set_block(
                Block::default()
                    .title("Email")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(if self.which == 0 {Color::Cyan} else {Color::Black}))
                    .border_style(Style::default().fg(Color::Green)),
            );
        } else {
            is_email_val = false;
            self.email.set_block(
                Block::default()
                    .title("Email")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(if self.which == 0 {Color::Cyan} else {Color::Black}))
                    .border_style(Style::default().fg(Color::Red)),
            );
        }
        let mut is_password_val = false;
        if !(self.password.lines()[0]).is_empty() {
            is_password_val = true;
            self.password.set_block(
                Block::default()
                    .title("Password")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(if self.which == 1 {Color::Cyan} else {Color::Black}))
                    .border_style(Style::default().fg(Color::Green)),
            );
        } else {
            is_password_val = false;
            self.password.set_block(
                Block::default()
                    .title("Password")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(if self.which == 1 {Color::Cyan} else {Color::Black}))
                    .border_style(Style::default().fg(Color::Red)),
            );
        }
        is_email_val & is_password_val
    }

    fn handle_input(&mut self) -> io::Result<()> {
        match crossterm::event::read()?.into() {
            // Input { key: Key::Tab, .. } => {
            //    self.which = (self.which + 1) % 4;
            // },
            Input { key: Key::Enter, .. } => {
                if self.which == 2 {
                    if self.is_valid() {
                        self.ch_popup_sender.send(PopupStatus::Add(AddPopupStatus::Save)).unwrap();
                        self.ch_popup_sender.send(PopupStatus::Add(AddPopupStatus::Exit)).unwrap();
                    }
                } else if self.which == 3 {
                    self.ch_popup_sender.send(PopupStatus::Add(AddPopupStatus::Exit)).unwrap();
                }
            },
            input => {
                if self.which == 0 {
                    self.email.input(input);
                } else if self.which == 1 {
                    self.password.input(input);
                };
            },
        }
        Ok(())
    }
}

fn get_popup_area(r: Rect) -> Rect {
    let [_, ver_area, _] = Layout::vertical([
            Constraint::Percentage((100 - 39)/2),
            Constraint::Percentage(39),
            Constraint::Percentage((100 - 39)/2),
        ])
        .areas(r);

    let [_, box_area, _] = Layout::horizontal([
            Constraint::Percentage((100 - 43)/2),
            Constraint::Percentage(43),
            Constraint::Percentage((100 - 43)/2),
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
        .constraints([
            Constraint::Fill(1),
            Constraint::Fill(1),
        ])
        .split(r);
    chunks
}
