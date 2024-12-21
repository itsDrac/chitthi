use crate::mail::Subject;
use crate::mail::Mailbox;

struct SubjectView<'mailbox> {
    mailbox: &'mailbox mut Mailbox,
    mail_count: u32,
    subjects: Vec<Subject>,
}

impl<'mailbox> SubjectView<'mailbox> {
    pub fn new(mailbox: &'mailbox mut Mailbox) -> Self {
        Self { 
            mailbox: mailbox,
            mail_count: 0,
        }
    }

    pub fn get_subjects(&mut self) -> Vec<String> {
        self.mailbox.get_subjects()
    }
}
