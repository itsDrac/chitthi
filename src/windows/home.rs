use std::io;
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
use crate::mail::{Mailbox};
use crate::chitthi::{Cred, AuthList};
use crate::components::FolderList;

enum Messages {
    UpdateFolderSelection,
}

#[derive(PartialEq)]
pub enum Sections {
    FolderList,
    MessageList,
    MessageView
}

pub struct HomePage {
    current_auth: Option<Cred>,
    current_section: Sections,
    ch_sender: mpsc::Sender<Messages>,
    ch_receiver: mpsc::Receiver<Messages>
}

impl HomePage {
    pub fn new() -> Self {
        let (ch_sender, ch_receiver) = mpsc::channel();
        Self { current_auth: AuthList::new().get_current(), current_section: Sections::FolderList, ch_sender, ch_receiver }
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
                    .constraints([
                        Constraint::Max(3),
                        Constraint::Fill(1),
                        Constraint::Max(3),
                    ])
                .split(frame.area());
            if let Ok(val) = self.ch_receiver.try_recv() {
                match val {
                    Messages::UpdateFolderSelection => {
                        folder_section.current_folder = (folder_section.current_folder + 1);
                    }
                }
            }
            let folder_list = folder_section.render_list(&self.current_section);
            frame.render_widget(folder_list, chunks[0]);
            })?;

            // handle key events such that when the user presses the 'Tab' key, the current section is changed

            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Tab {
                        self.current_section = match self.current_section {
                            Sections::FolderList => Sections::MessageList,
                            Sections::MessageList => Sections::MessageView,
                            Sections::MessageView => Sections::FolderList,
                        };
                    }
                    else if key.code == KeyCode::Char('d') {
                        match self.current_section {
                            Sections::FolderList => {
                                self.ch_sender.send(Messages::UpdateFolderSelection);
                            },
                            _ => {},
                        }
                    }
                    else if key.code == KeyCode::Char('q') {
                        return Ok(());
                    }
                }
            }
        }
    }
}


