use crate::mail::MailboxMessageType;
use crate::mail::Subject;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{Block, List, ListItem, ListState},
};
use std::sync::mpsc;

pub struct SubjectView {
    mailbox_sender: mpsc::Sender<MailboxMessageType>,
    pub mail_count: usize,
    pub subjects: Vec<Subject>,
    pub current_hover: ListState,
    pub focused: bool,
}

impl SubjectView {
    pub fn new(mailbox_sender: mpsc::Sender<MailboxMessageType>) -> Self {
        Self {
            mailbox_sender,
            mail_count: 5,
            subjects: Vec::new(),
            current_hover: ListState::default(),
            focused: false,
        }
    }

    pub fn clear_subjects(&mut self) {
        self.subjects.clear();
    }

    pub fn update_subjects(&mut self, mut subjects: Vec<Subject>) {
        self.subjects.append(&mut subjects);
    }

    pub fn hover_next(&mut self) {
        match self.current_hover.selected() {
            Some(_) => self.current_hover.select_next(),
            None => self.current_hover.select_first(),
        }
    }

    pub fn hover_previous(&mut self) {
        if let Some(_) = self.current_hover.selected() {
            self.current_hover.select_previous();
        }
    }

    fn get_subject_line(&self, subject: Subject) -> Text {
        // Make a Line widget for subject
        let subject_text = Line::from(subject.subject.to_string()).style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        );
        // Make a Line widget for date
        let date_text = Line::from(subject.date.to_string())
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::ITALIC),
            )
            .right_aligned();
        // Make a Paragraph widget using the above two widgets
        let subject_text = Text::from(vec![subject_text, Line::default(), date_text]);
        subject_text
    }

    pub fn render_list(&mut self) -> List {
        let mut paragraph_items: Vec<ListItem> = Vec::new();
        for (i, subject) in self.subjects.iter().enumerate() {
            let subject_line = self.get_subject_line(subject.clone());
            let item = ListItem::new(subject_line);
            paragraph_items.push(item);
        }
        // Make List widget which takes in vector of listItems
        let list = List::new(paragraph_items)
            .block(Block::bordered().title("Subjects"))
            .highlight_style(Style::default().bg(Color::Cyan))
            .style(Style::default().fg(if self.focused {
                Color::Cyan
            } else {
                Color::White
            }))
            .scroll_padding(1);
        list
    }
}
