extern crate imap;
extern crate native_tls;
use crate::chitthi::{Cred};
use std::io;
use native_tls::TlsStream;
use std::net::TcpStream;

pub struct Mailbox {
    domain: String,
    email: String,
    password: String,
    session: Option<imap::Session<TlsStream<TcpStream>>>
}

impl Mailbox {

    pub fn new(cred: Cred) -> Self {
        Self {
            domain: "imap.gmail.com".to_string(),
            email: cred.email,
            password: cred.password,
            session: None
        }
    }

    pub fn connect(&mut self) -> Result<(), imap::Error> {
        let tls = native_tls::TlsConnector::builder().build().unwrap();
        let client = imap::connect((self.domain.as_str(), 993), &self.domain, &tls).unwrap();
        let imap_session = client
            .login(&self.email, &self.password)
            .map_err(|e| e.0)?;
        
        self.session = Some(imap_session);
        Ok(())
    }

    pub fn list_folders(&mut self) -> Result<Vec<String>, imap::Error> {
        match &mut self.session {
            Some(session) => {
                let folders: imap::types::ZeroCopy<Vec<imap::types::Name>> = session.list(None, Some("*"))?;
                let folder_name: Vec<String> = folders.iter()
                    .map(|folder| folder.name().to_string()).collect();
                Ok(folder_name)
            },
            None => Err(imap::Error::No("Session not found, Please connect to imap".to_string()))
        }
    }

    pub fn fetch_inbox_top(self) -> imap::error::Result<Option<String>> {
        let domain = "imap.gmail.com";
        let tls = native_tls::TlsConnector::builder().build().unwrap();
        let client = imap::connect((domain, 993), domain, &tls).unwrap();
        let mut imap_session = client
            .login(&self.email, &self.password)
            .map_err(|e| e.0)?;
        imap_session.select("[Gmail]")?;
        let messages = imap_session.fetch("1", "RFC822")?;
        let message = if let Some(m) = messages.iter().next() {
            m
        } else {
            return Ok(None);
        };

        let body = message.body().expect("message did not have a body!");
        let body = std::str::from_utf8(body)
            .expect("message was not valid utf-8")
            .to_string();

        imap_session.logout()?;

        Ok(Some(body))
    }
}
