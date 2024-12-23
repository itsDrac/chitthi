extern crate imap;
extern crate native_tls;
use crate::chitthi::Cred;
use chrono::{DateTime, Utc};
use mail_parser::MessageParser;
use native_tls::TlsStream;
use std::net::TcpStream;

pub struct Mailbox {
    domain: String,
    email: String,
    password: String,
    session: Option<imap::Session<TlsStream<TcpStream>>>,
}

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
    pub fn new(cred: Cred) -> Self {
        Self {
            domain: "imap.gmail.com".to_string(),
            email: cred.email,
            password: cred.password,
            session: None,
        }
    }

    pub fn connect(&mut self) -> Result<(), imap::Error> {
        let tls = native_tls::TlsConnector::builder().build().unwrap();
        let client = imap::connect((self.domain.as_str(), 993), &self.domain, &tls).unwrap();
        let imap_session = client.login(&self.email, &self.password).map_err(|e| e.0)?;

        self.session = Some(imap_session);
        Ok(())
    }

    pub fn list_folders(&mut self) -> Result<Vec<String>, imap::Error> {
        match &mut self.session {
            Some(session) => {
                let folders: imap::types::ZeroCopy<Vec<imap::types::Name>> =
                    session.list(None, Some("*"))?;
                let folder_name: Vec<String> = folders
                    .iter()
                    .map(|folder| folder.name().to_string())
                    .collect();
                Ok(folder_name)
            }
            None => Err(imap::Error::No(
                "Session not found, Please connect to imap".to_string(),
            )),
        }
    }

    pub fn select_folder(&mut self, folder: &str) -> Result<(), imap::Error> {
        let mut session = self.session.as_mut().unwrap();
        session.select(folder)?;
        Ok(())
    }

    pub fn get_next_set_of_subjects(&mut self, mail_count: usize) -> Vec<Subject> {
        let mut session = self.session.as_mut().unwrap();
        let mut subjects: Vec<Subject> = Vec::new();
        let mail_ids = mail_count - 15..=mail_count;
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
}
