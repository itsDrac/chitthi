use crate::chitthi::AuthList;
use crate::mail::{mailbox as mb, Mailbox};
use crate::windows::screens;
use std::io;

mod chitthi;
mod components;
mod mail;
mod windows;

fn main() -> io::Result<()> {
    // let mut auth = AuthList::new();
    // let cred = auth.get_current().unwrap();
    // let mut mb = Mailbox::new();
    // mb.connect(cred).unwrap();
    // let sender: std::sync::mpsc::Sender<mb::MailboxMessageType> = mb.get_sender();
    // sender.send(mb::MailboxMessageType::ListFolders).unwrap();
    // mb.listen_message();
    // let folders = mb.folders.clone();
    // println!("{folders:#?}");
    // sender.send(mb::MailboxMessageType::SelectFolder(String::from("[Gmail]/Sent Mail"))).unwrap();
    // mb.listen_message();
    // sender.send(mb::MailboxMessageType::GetMoreSubjects(2)).unwrap();
    // mb.listen_message();
    // let subjects = mb.subjects.clone();
    // println!("{subjects:#?}");
    // sender.send(mb::MailboxMessageType::GetMoreSubjects(4)).unwrap();
    // mb.listen_message();
    // let subjects = mb.subjects.clone();
    // println!("{subjects:#?}");
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = screens::start(&mut terminal);
    ratatui::restore();
    app_result;
    Ok(())
}
