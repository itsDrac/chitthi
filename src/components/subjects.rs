use crate::mail::Subject;
use crate::mail::Mailbox;
use ratatui::{
    style::{Style, Color, Modifier},
    text::{Line, Span},
    widgets::{List, ListItem},
};

pub struct SubjectView<'mailbox> {
    mailbox: &'mailbox mut Mailbox,
    mail_count: u32,
    subjects: Vec<Subject>,
}

impl<'mailbox> SubjectView<'mailbox> {
    pub fn new(mailbox: &'mailbox mut Mailbox) -> Self {
        Self { 
            mailbox: mailbox,
            mail_count: 0,
            subjects: Vec::new()
        }
    }

    pub fn update_subjects(&mut self) {
        self.mail_count += 15;
        self.subjects.append(&mut self.mailbox.get_next_set_of_subjects(self.mail_count as usize));
    }

    fn get_subject_line(&mut self, subject: &Subject) -> Line {
        // TODO: Make a Line widget for subject
        let subject_text = Span::from(subject.subject.to_string())
            .style(Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD));
        // TODO: Make a Line widget for date
        let date_text = Span::from(subject.date.to_string())
            .style(Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::ITALIC));
        // TODO: Make a Paragraph widget using the above two widgets
        let subject_line = Line::from(
            vec![subject_text, Span::from(String::from("\n")), date_text]
        );
        subject_line
    }

    pub fn render_list(&mut self) -> List {

    let mut paragraph_items: Vec<ListItem> = Vec::new();
    for subject in self.subjects.iter_mut() {
        let subject_line = self.get_subject_line(subject);
        paragraph_items.push(ListItem::new(subject_line));
    }
    // TODO: Make List widget which takes in vector of listItems
    let list = List::new(paragraph_items);
    list
    }
}


