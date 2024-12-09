extern crate dirs;
use std::fs::{self, File, OpenOptions};
use std::path::{Path, PathBuf};
use std::io::{self, Read, Write};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use std::error::{Error};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Cred {
    pub id: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthList {
    pub auths: Vec<Cred>,
    pub current: Option<String>
}

pub struct Config;

impl Cred {
    pub fn new(email: String, password: String) -> Self {
        let id = Self::get_hash(&email);
        Self {
            id: id.to_string(),
            email: email,
            password: password
        }
    }

    fn get_hash(email: &str) -> u64 {
        let mut hasher = DefaultHasher::new();
        email.hash(&mut hasher);
        hasher.finish()
    }
}

impl AuthList {
    pub fn new() -> Self {
        let mut contents = String::new();
        let mut config_file = Config::get_file(true, false).expect("can not get file");
        config_file.read_to_string(&mut contents).expect("Can not read from file");
        if !contents.trim().is_empty() {
            toml::from_str(&contents).expect("Can not read config file")
        } else {
            AuthList {
                auths: Vec::new(),
                current: None
            }
        }
    }

    pub fn add_cred(&mut self, cred: &Cred) {
        self.auths.push(cred.clone());
    }

    pub fn set_current(&mut self, cred: &Cred) {
        self.current = Some(cred.id.clone());
    }

    pub fn write_file(&mut self) {
        let mut config_file = Config::get_file(false, true).expect("can not get file");
        let mut auth_string = toml::to_string_pretty(self).expect("Can not convert to string");
        config_file.write(auth_string.as_bytes()).expect("Can not write to file");
    }

}

impl Config {
    fn is_exist() -> bool {
        let config_dir = Self::get_path();
        config_dir.exists()
    }

    fn create() {
        let config_dir = Self::get_path();
        let chitthi_folder: &Path = config_dir.as_path();
        fs::create_dir(chitthi_folder)
            .expect("Can not create folder, Please check permission");
    }

    fn get_path() -> PathBuf {
        let mut config_dir: PathBuf = dirs::config_local_dir().expect("Can not access local config directory");
        config_dir.push(Path::new("chitthi"));
        config_dir
    }

    pub fn make_file() {
        if !Self::is_exist() {
            Self::create();
        }
        let mut file_path = Self::get_path();
        file_path.push(Path::new("auth.toml")); 
        let mut f = File::create(file_path.to_str().unwrap()).expect("Can not create configration file");
    }
    
    fn get_file(readable: bool, writeable: bool) -> io::Result<File> {
        let mut file_path: PathBuf = Self::get_path();
        file_path.push(Path::new("auth.toml"));
        let file_path: &str = file_path.to_str().unwrap();
        Ok(OpenOptions::new()
            .read(readable)
            .write(writeable)
            .truncate(!readable)
            .open(file_path)?)
    }

    pub fn is_file_exist() -> io::Result<bool> {
        let mut file_path: PathBuf = Self::get_path();
        file_path.push(Path::new("auth.toml"));
        Ok(file_path.exists())
    }
}
