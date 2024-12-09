use crate::chitthi::{Config, Cred, AuthList};
use crate::windows::welcome;
use ratatui::{
    DefaultTerminal,
};
use std::{io};

pub enum Screens<'term> {
    Welcome(&'term mut DefaultTerminal)
}

impl<'term> Screens<'term> {
    fn run(&mut self) {
        match self {
            Screens::Welcome(t) => welcome::run(t).expect("Can not run the welcome window."),
        }
    }
}

pub fn start(terminal: &mut DefaultTerminal) {
    if !Config::is_file_exist().expect("Can not get config file, Please check permission") {
        Config::make_file();
    }
    run(terminal).expect("Unable to run the program");
}

fn run(terminal: &mut DefaultTerminal) -> io::Result<()> {
    let mut auth_list: AuthList = AuthList::new();
    match auth_list.current {
        // Some(val) => println!("WIP {:?}", val),
        _ => {
            let mut screen = Screens::Welcome(terminal);
            screen.run();
        }
    }
    Ok(())
}
