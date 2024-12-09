extern crate dirs;
use std::io;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::mpsc;
use ratatui::{
    prelude::{Layout, Direction, Constraint},
    crossterm::event::{self, KeyCode, KeyEventKind},
    style::Stylize,
    widgets::{Paragraph, Block, Borders},
    text::{Line},
    symbols::{border},
    DefaultTerminal,
    layout::Alignment,
};
use crate::chitthi::{Config, Cred, AuthList};
use crate::components::{AddCredPopup, AddPopupStatus};

enum Popups<'text_area> {
    Add(AddCredPopup<'text_area>),
    View,
    Quit,
}

pub fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut current_popup: Option<Popups> = None;
    let mut listion_for_input = true;
    let (ch_popup_sender, ch_popup_receiver) = mpsc::channel();
    loop {
        terminal.draw(|frame| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(3),
                    Constraint::Percentage(25),
                ])
                .split(frame.area());
            let greeting = Paragraph::new("Hello! welcome to chitthi, your in-terminal mail manager")
                .block(Block::bordered())
                .white()
                .on_cyan()
                .alignment(Alignment::Center);
            let instruction = Line::from(vec![
                " Add new account: ".into(),
                "<A>".green().bold(),
                " View added accounts: ".into(),
                "<V>".green().bold(),
                " Quit: ".into(),
                "<Q> ".red().bold()
            ]);
            let bottom_block = Block::bordered()
                .title_bottom(instruction.white().centered())
                .border_set(border::DOUBLE);
            frame.render_widget(greeting, chunks[0]);
            frame.render_widget(bottom_block, chunks[1]);
            if let Some(popup) = &mut current_popup {
                match popup {
                    Popups::Add(add_popup) => {
                        add_popup.draw(frame);
                    },
                    _ => println!("WIP"),
                }
            }
        })?;

        if listion_for_input {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') {
                        return Ok(());
                    } else if key.code == KeyCode::Char('a') {
                        ch_popup_sender.send(AddPopupStatus::Show).unwrap();
                    }
                }
            }
        }
        if let Ok(val) = ch_popup_receiver.try_recv() {
            match val {
                AddPopupStatus::Show => {
                    current_popup = Some(Popups::Add(
                            AddCredPopup::new(ch_popup_sender.clone())
                            ));
                    listion_for_input = false;
                },
                AddPopupStatus::Exit => {
                    current_popup = None;
                    listion_for_input = true;
                },
                AddPopupStatus::Save => {
                    if let Some(popup) = &mut current_popup {
                        if let Popups::Add(add_popup) = popup {
                            let email = &add_popup.email.lines()[0];
                            let password = &add_popup.password.lines()[0];
                            let new_cred = Cred::new(email.to_string(), password.to_string());
                            let mut auth_list: AuthList = AuthList::new();
                            auth_list.add_cred(&new_cred);
                            auth_list.set_current(&new_cred);
                            auth_list.write_file();
                        }
                    }
                },
            }
        }
    }
}
