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

pub enum Sections {
    FolderList,
    SubjectList,
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
            // handle messages
            mailbox.listen_message();
            terminal.draw(|frame| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Max(3), Constraint::Fill(1), Constraint::Max(3)])
                    .split(frame.area());

                folder_section.set_folders(mailbox.folders.clone());
                let folder_list = folder_section.render_list();
                frame.render_widget(folder_list, chunks[0]);
                // Divide chunks[1] into 2 horizontal chunks
                let mail_chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(30), Constraint::Fill(1)])
                    .split(chunks[1]);
                let mut subject_current_hover =
                    std::mem::replace(&mut subject_section.current_hover, Default::default());
                if mailbox.new_subjects_available {
                    subject_section.update_subjects(mailbox.subjects.clone());
                    mailbox.new_subjects_available = false;
                }
                let subject_list = subject_section.render_list();
                frame.render_stateful_widget(
                    subject_list,
                    mail_chunks[0],
                    &mut subject_current_hover,
                );
                subject_section.current_hover = subject_current_hover;
            })?;

            // handle key events such that when the user presses the 'Tab' key, the current section is changed
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Tab {
                        self.current_section = match self.current_section {
                            Sections::FolderList => {
                                folder_section.focused = false;
                                Sections::SubjectList
                            }
                            Sections::SubjectList => {
                                subject_section.focused = false;
                                Sections::MessageView
                            }
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
                                folder_section.update_hover(true);
                            }

                            _ => {}
                        }
                    } else if key.code == KeyCode::Char('h') {
                        match self.current_section {
                            Sections::FolderList => {
                                folder_section.update_hover(false);
                            }

                            _ => {}
                        }
                    } else if key.code == KeyCode::Char('j') {
                        match self.current_section {
                            Sections::SubjectList => {
                                subject_section.hover_next();
                                if let Some(selected_index) =
                                    subject_section.current_hover.selected()
                                {
                                    if selected_index == (subject_section.mail_count - 2) {
                                        subject_section.mail_count += 5;
                                        let _ = mailbox_sender.send(
                                            MailboxMessageType::GetMoreSubjects(
                                                subject_section.mail_count,
                                            ),
                                        );
                                    }
                                }
                            }

                            _ => {}
                        }
                    } else if key.code == KeyCode::Char('k') {
                        match self.current_section {
                            Sections::SubjectList => {
                                subject_section.hover_previous();
                            }

                            _ => {}
                        }
                    } else if key.code == KeyCode::Enter {
                        match self.current_section {
                            Sections::FolderList => {
                                self.current_section = Sections::SubjectList;
                                folder_section.update_selection();
                                folder_section.focused = false;
                                let _ = mailbox_sender.send(MailboxMessageType::GetMoreSubjects(
                                    subject_section.mail_count,
                                ));
                                subject_section.clear_subjects();
                                subject_section.focused = true;
                            }

                            _ => {}
                        }
                    } else if key.code == KeyCode::Esc {
                        return Ok(());
                    }
                }
            }
        }
    }
}
