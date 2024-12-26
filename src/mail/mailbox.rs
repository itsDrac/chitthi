extern crate imap;
use crate::chitthi::Cred;
use chrono::{DateTime, Utc};
use mail_parser::MessageParser;
use std::sync::mpsc;

pub enum MailboxMessageType {
    ListFolders,
    SelectFolder(String),
    GetMoreSubjects(u32),
    Quit,
}

pub struct Mailbox {
    mailbox: Option<imap::types::Mailbox>,
    pub folders: Vec<String>,
    session: Option<imap::Session<Box<dyn imap::ImapConnection>>>,
    pub subjects: Vec<Subject>,
    sender: mpsc::Sender<MailboxMessageType>,
    receiver: mpsc::Receiver<MailboxMessageType>,
}

#[derive(Debug, Clone)]
pub struct Subject {
    pub id: u32,
    pub subject: String,
    pub date: String,
}

impl Subject {
    fn convert_date_to_readable(date: i64) -> String {
        let date = DateTime::<Utc>::from_timestamp(date, 0).unwrap();
        let date = date.format("%d/%b/%Y %r").to_string();
        date
    }
}

impl Mailbox {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            session: None,
            sender,
            receiver,
            folders: Vec::new(),
            subjects: Vec::new(),
            mailbox: None,
        }
    }

    pub fn connect(&mut self, cred: Cred) -> Result<(), imap::Error> {
        let domain = "imap.gmail.com";
        let client = imap::ClientBuilder::new(domain, 993).connect()?;
        let imap_session = client.login(&cred.email, &cred.password).map_err(|e| e.0)?;
        self.session = Some(imap_session);
        Ok(())
    }

    fn set_folders(&mut self) -> Result<(), imap::Error> {
        let mut session = self.session.as_mut().unwrap();
        let folders: imap::types::Names = session.list(None, Some("*"))?;
        let folder_name: Vec<String> = folders
            .iter()
            .map(|folder| folder.name().to_string())
            .collect();
        self.folders = folder_name;
        Ok(())
    }

    pub fn select_folder(&mut self, folder: &str) -> Result<(), imap::Error> {
        let mut session = self.session.as_mut().unwrap();
        self.mailbox = Some(session.select(folder)?);
        Ok(())
    }

    pub fn get_more_subjects(&mut self, mail_count: u32) -> Vec<Subject> {
        let mut session = self.session.as_mut().unwrap();
        let mut subjects: Vec<Subject> = Vec::new();
        let mailbox = self.mailbox.as_mut().unwrap();
        let mail_ids = mailbox.exists - (mail_count - 1)..=mailbox.exists;
        let mail_ids: Vec<String> = mail_ids.map(|id| id.to_string()).collect();
        let mail_ids = mail_ids.join(",");
        let mails = session
            .fetch(mail_ids, "BODY[HEADER.FIELDS (SUBJECT DATE)]")
            .unwrap();
        for mail in mails.iter() {
            subjects.push(Self::get_subject_from_mail(mail));
        }
        subjects
    }

    fn get_subject_from_mail(mail: &imap::types::Fetch) -> Subject {
        let message = MessageParser::default()
            .parse_headers(mail.header().unwrap())
            .unwrap();
        Subject {
            id: mail.message,
            subject: message.subject().unwrap_or("").to_string(),
            date: Subject::convert_date_to_readable(message.date().unwrap().to_timestamp()),
        }
    }

    pub fn get_sender(&self) -> mpsc::Sender<MailboxMessageType> {
        self.sender.clone()
    }

    pub fn listen_message(&mut self) {
        let Ok(message) = self.receiver.try_recv() else {
            return;
        };
        match message {
            MailboxMessageType::ListFolders => {
                self.set_folders().unwrap();
            }
            MailboxMessageType::SelectFolder(folder) => {
                self.select_folder(&folder).unwrap();
            }
            MailboxMessageType::GetMoreSubjects(mail_count) => {
                self.subjects = self.get_more_subjects(mail_count);
            }
            MailboxMessageType::Quit => {
                self.session = None;
            }
        }
    }
}
