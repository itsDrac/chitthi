use crate::mail::MailboxMessageType;
use ratatui::{
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};
use std::sync::mpsc;

pub struct FolderList {
    mailbox_sender: mpsc::Sender<MailboxMessageType>,
    pub current_folder: u8,
    pub hover_folder: u8,
    pub focused: bool,
    folders: Vec<String>,
}

impl FolderList {
    pub fn new(mailbox_sender: mpsc::Sender<MailboxMessageType>) -> Self {
        Self {
            mailbox_sender,
            current_folder: u8::MAX,
            hover_folder: u8::MIN,
            focused: false,
            folders: Vec::new(),
        }
    }

    pub fn set_folders(&mut self, folders: Vec<String>) {
        let folders: Vec<String> = folders
            .into_iter()
            .filter(|folder| folder != "[Gmail]")
            .collect();
        self.folders = folders;
    }

    pub fn update_hover(&mut self, to_right: bool) {
        if to_right {
            self.hover_folder = (self.hover_folder + 1) % self.folders.len() as u8;
        } else {
            self.hover_folder = (self.hover_folder - 1) % self.folders.len() as u8;
        }
    }

    pub fn update_selection(&mut self) {
        self.current_folder = self.hover_folder;
        self.hover_folder = u8::MIN;
        let _ = self.mailbox_sender.send(MailboxMessageType::SelectFolder(
            self.folders[self.current_folder as usize].clone(),
        ));
    }

    fn get_spans(&mut self) -> Vec<Span> {
        // let folders = self.mailbox.list_folders().unwrap();

        let folders: Vec<String> = self
            .folders
            .clone()
            .into_iter()
            .map(|folder| {
                let folder = folder.split("/").last().unwrap_or("");
                folder.to_string()
            })
            .collect();

        // Create a vector of span widgets where folder where indexed hover_folder is of green color

        let spans: Vec<Span> = folders
            .into_iter()
            .enumerate()
            .map(|(i, folder)| {
                let mut span = Span::default();
                if self.focused {
                    if i == self.hover_folder as usize {
                        span = span.style(Style::default().fg(Color::Green));
                    }
                }
                if i == self.current_folder as usize {
                    span = span.style(Style::default().bg(Color::Cyan));
                }
                span.content(folder)
            })
            .collect();

        spans
    }

    pub fn render_list(&mut self) -> Paragraph {
        // Create a Block widget with the line and the title "Folders", also Block border should be Clay when focused is True.

        let mut block = Block::default().title("Folders").borders(Borders::ALL);

        // create a if block is focused, then border should be clay, else white

        if self.focused {
            block = block.border_style(Style::default().fg(Color::Cyan));
        } else {
            block = block.border_style(Style::default().fg(Color::White));
        };
        // get the list of folders from the mailbox
        let spans = self.get_spans();
        // Create a Line widget with the spams with " | " between them

        let mut spans_with_spaces: Vec<Span> = Vec::new();

        for span in spans.into_iter() {
            spans_with_spaces.push(span);

            spans_with_spaces.push(Span::from(" | "));
        }

        let line = Line::from(spans_with_spaces);

        // Create a Paragraph widget with the block

        let paragraph = Paragraph::new(line)
            .wrap(Wrap { trim: false })
            .block(block)
            .left_aligned();
        paragraph
    }
}
