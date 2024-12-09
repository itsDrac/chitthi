use crate::windows::{screens};
use crate::mail::mailbox;
use crate::chitthi::{Config, Cred, AuthList};
use std::io;
use ratatui;

mod windows;
mod mail;
mod chitthi;
mod components;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = screens::start(&mut terminal);
    ratatui::restore();
    app_result;
    Ok(())
}
