use std::io;

use std::sync::mpsc;

use crate::mail::{Mailbox, MailboxMessageType};
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

use crate::components;

enum Messages {
    UpdateFolderHover,
    UpdateFolderSelection,
    ChangeSection,
}

pub enum Sections {
    FolderList,
    MessageList,
    MessageView,
}

pub struct HomePage {
    current_section: Sections,
}

impl HomePage {
    pub fn new() -> Self {
        Self {
            current_section: Sections::FolderList,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let cred = AuthList::new().get_current().unwrap();
        let mut mailbox = Mailbox::new();
        mailbox.connect(cred).unwrap();
        let mailbox_sender: std::sync::mpsc::Sender<MailboxMessageType> = mailbox.get_sender();

        // Draw the folder list

        let mut folder_section = components::FolderList::new(mailbox.get_sender());
        folder_section.focused = true;
        let _ = mailbox_sender.send(MailboxMessageType::ListFolders);
        let mut subject_section = components::SubjectView::new(mailbox.get_sender());
        loop {
            terminal.draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Max(3), Constraint::Fill(1), Constraint::Max(3)])
                    .split(frame.area());

                folder_section.set_folders(mailbox.folders.clone());
                let folder_list = folder_section.render_list();
                frame.render_widget(folder_list, chunks[0]);
                let subject_list = subject_section.render_list();
                frame.render_widget(subject_list, chunks[1]);
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
                                let _ = mailbox_sender
                                    .send(MailboxMessageType::ListFolders)
                                    .unwrap();
                                folder_section.focused = true;
                                Sections::FolderList
                            }
                        };
                    } else if key.code == KeyCode::Char('l') {
                        match self.current_section {
                            Sections::FolderList => {
                                folder_section.update_hover();
                            }

                            _ => {}
                        }
                    } else if key.code == KeyCode::Enter {
                        match self.current_section {
                            Sections::FolderList => {
                                self.current_section = Sections::MessageList;
                                folder_section.update_selection();
                                folder_section.focused = false;
                                let _ = mailbox_sender.send(MailboxMessageType::GetMoreSubjects(
                                    subject_section.mail_count,
                                ));
                                subject_section.update_subjects(mailbox.subjects.clone());
                            }

                            _ => {}
                        }
                    } else if key.code == KeyCode::Esc {
                        return Ok(());
                    }
                }
            }
            // handle messages
            mailbox.listen_message();
        }
    }
}
