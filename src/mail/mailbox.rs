extern crate imap;
extern crate native_tls;

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

    fn new(email: String, password: String, domain: String) -> Self {
        Self {
            domain,
            email,
            password,
            session: None
        }
    }

    fn connect(&mut self) -> Result<(), imap::Error> {
        let tls = native_tls::TlsConnector::builder().build().unwrap();
        let client = imap::connect((self.domain.as_str(), 993), &self.domain, &tls).unwrap();
        let mut imap_session = client
            .login(&self.email, &self.password)
            .map_err(|e| e.0)?;
        self.session = Some(imap_session);
        Ok(())
    }

    fn list_folders(self) -> Result<Vec<String>, imap::Error> {
        match self.session {
            Some(mut session) => {
                let folders: imap::types::ZeroCopy<Vec<imap::types::Name>> = session.list(None, Some("*"))?;
                let folder_name: Vec<String> = folders.iter()
                    .map(|folder| folder.name().to_string()).collect();
                Ok(folder_name)
            },
            None => Err(imap::Error::No("Session not found, Please connect to imap".to_string()))
        }
    }
}

pub fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
    let domain = "imap.gmail.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, 993), domain, &tls).unwrap();
    let mut imap_session = client
        .login("gpt.sahaj28@gmail.com", "ojhj cnyv ygdt vxsr")
        .map_err(|e| e.0)?;
    let folder_name = list_folders(&mut imap_session);
    println!("{:?}", folder_name);
    imap_session.select("INBOX")?;
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

pub fn list_folders<T: io::Read + io::Write>(session: &mut imap::Session<T>) -> Result<Vec<String>, imap::Error> {
    let folders: imap::types::ZeroCopy<Vec<imap::types::Name>> = session.list(None, Some("*"))?;
    let folder_name: Vec<String> = folders.iter()
        .map(|folder| folder.name().to_string()).collect();
    Ok(folder_name)
}
