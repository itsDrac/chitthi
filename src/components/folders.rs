use ratatui::{
    widgets::{Tabs, Block, Borders},
    style::{Style, Color},
};
use crate::mail::Mailbox;
use crate::windows::home::Sections;

pub struct FolderList<'mailbox> {
    mailbox: &'mailbox mut Mailbox,
    pub tabs: Option<Tabs<'mailbox>>,
    pub current_folder: u8
}

impl<'mailbox> FolderList<'mailbox> {
    pub fn new(mailbox: &'mailbox mut Mailbox) -> Self {
        Self {
            mailbox,
            tabs: None,
            current_folder: 0
        }
    }

    pub fn render_list(&mut self, current_section: &Sections) -> &Tabs<'mailbox> {
        // get the list of folders from the mailbox
        let folders = self.mailbox.list_folders().unwrap();
        // Create a vector of folder names, excluding "[Gmail]" (note the correct casing and brackets)
        let folder_names: Vec<String> = folders
            .into_iter()
            .filter(|folder| folder != "[Gmail]")
            .collect();
        let folder_length = folder_names.len() as u8;
        // create a Tabs widget and render the list of folders
        self.tabs = Some(Tabs::new(folder_names)
            .select((self.current_folder % folder_length) as usize)
            .highlight_style(Style::default().bg(Color::Cyan))
            .block(Block::default()
                .title("Folders")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(
                    if *current_section == Sections::FolderList {
                        Color::Cyan
                    } else {
                        Color::White
                    }
                ))
            ));
        self.tabs.as_ref().unwrap()
    }

}
