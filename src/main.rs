use crate::windows::screens;
use std::io;

mod chitthi;
mod components;
mod mail;
mod types;
mod windows;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = screens::start(&mut terminal);
    ratatui::restore();
    Ok(())
}
