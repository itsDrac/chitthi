use crate::chitthi::{AuthList, Config, Cred};
use crate::mail::{mailbox as mb, Mailbox};
use crate::windows::screens;
use ratatui;
use std::io;

mod chitthi;
mod components;
mod mail;
mod windows;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    terminal.clear()?;
    let app_result = screens::start(&mut terminal);
    ratatui::restore();
    app_result;
    Ok(())
}
