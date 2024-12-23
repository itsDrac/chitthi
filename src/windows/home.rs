use std::io;

use std::sync::mpsc;

use crate::mail::Mailbox;
use ratatui::{
    crossterm::event::{self, KeyCode, KeyEventKind},
    layout::Alignment,
    prelude::{Constraint, Direction, Layout},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Borders, Paragraph},
    DefaultTerminal,
};

use crate::chitthi::{AuthList, Cred};

use crate::components::FolderList;

enum Messages {
    UpdateFolderHover,

    UpdateFolderSelection,

    ChangeSection(Sections),
}

#[derive(PartialEq)]

pub enum Sections {
    FolderList,

    MessageList,

    MessageView,
}

pub struct HomePage {
    current_auth: Option<Cred>,

    current_section: Sections,

    ch_sender: mpsc::Sender<Messages>,

    ch_receiver: mpsc::Receiver<Messages>,
}

impl HomePage {
    pub fn new() -> Self {
        let (ch_sender, ch_receiver) = mpsc::channel();

        Self {
            current_auth: AuthList::new().get_current(),
            current_section: Sections::FolderList,
            ch_sender,
            ch_receiver,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let mut mailbox = Mailbox::new(self.current_auth.as_ref().unwrap().clone());

        mailbox.connect().unwrap();

        // Draw the folder list

        let mut folder_section = FolderList::new(&mut mailbox);
        loop {
            terminal.draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Max(3), Constraint::Fill(1), Constraint::Max(3)])
                    .split(frame.area());

                let folder_list = folder_section.render_list();

                frame.render_widget(folder_list, chunks[0]);
            })?;

            // handle key events such that when the user presses the 'Tab' key, the current section is changed
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Tab {
                        self.current_section = match self.current_section {
                            Sections::FolderList => {
                                folder_section.focused = false;
                                Sections::MessageList
                            }
                            Sections::MessageList => Sections::MessageView,
                            Sections::MessageView => {
                                folder_section.focused = true;
                                Sections::FolderList
                            }
                        };
                    } else if key.code == KeyCode::Char('l') {
                        match self.current_section {
                            Sections::FolderList => {
                                if folder_section.focused {
                                    let _ = self.ch_sender.send(Messages::UpdateFolderHover);
                                }
                            }

                            _ => {}
                        }
                    } else if key.code == KeyCode::Enter {
                        match self.current_section {
                            Sections::FolderList => {
                                let _ = self.ch_sender.send(Messages::UpdateFolderSelection);
                                self.current_section = Sections::MessageList;
                            }

                            _ => {}
                        }
                    } else if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
            }
            // handle messages
            if let Ok(val) = self.ch_receiver.try_recv() {
                match val {
                    Messages::UpdateFolderHover => {
                        folder_section.update_hover();
                    }
                    Messages::UpdateFolderSelection => {
                        folder_section.update_selection();
                    }
                    _ => {}
                }
            }
        }
    }
}
