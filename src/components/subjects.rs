use crate::mail::MailboxMessageType;
use crate::mail::Subject;
use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Text},
    widgets::{List, ListItem},
};
use std::sync::mpsc;

pub struct SubjectView {
    mailbox_sender: mpsc::Sender<MailboxMessageType>,
    pub mail_count: u32,
    subjects: Vec<Subject>,
    selected_mail: usize,
    focused: bool,
}

impl SubjectView {
    pub fn new(mailbox_sender: mpsc::Sender<MailboxMessageType>) -> Self {
        Self {
            mailbox_sender,
            mail_count: 5,
            subjects: Vec::new(),
            selected_mail: 0,
            focused: false,
        }
    }

    pub fn update_subjects(&mut self, mut subjects: Vec<Subject>) {
        self.subjects.append(&mut subjects);
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
            let mut item = ListItem::new(subject_line);
            if self.selected_mail == i {
                item = item.style(Style::default().bg(Color::Cyan));
            }
            paragraph_items.push(item);
        }
        // Make List widget which takes in vector of listItems
        let list = List::new(paragraph_items);
        list
    }
}
