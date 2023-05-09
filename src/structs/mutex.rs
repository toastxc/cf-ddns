use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mutex {
    pub state: MutexState,
    pub path: String,
    pub timeout: u64,
}

impl Mutex {
    // creates a mutex based on a toml file
    // if mutex is locked before creation it will panic
    pub async fn gen(&self) -> Result<Self, MutexError> {
        let mut watchdog = 0;
        loop {
            match MutexState::from_path(&self.path) {
                // if unlocked, leave loop
                Ok(read_state) => {
                    if !read_state.is_locked() {
                        break;
                    }
                }
                // if locked, wait
                Err(MutexError::State(MutexExceptions::AlreadyLocked)) => {
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }

                // otherwise throw error
                Err(other_error) => return Err(other_error),
            }
            watchdog += 1;
            if watchdog > self.timeout {
                // timeout waiting, exceeded limit for mutexs
                return Err(MutexError::State(MutexExceptions::AlreadyLocked));
            }
        }

        Ok(self.to_owned())
    }

    // gen alias
    pub async fn generate(&self) -> Result<Self, MutexError> {
        self.gen().await
    }

    pub fn new() -> Self {
        // defaults
        Self {
            state: MutexState::Locked,
            path: String::from(".mutex.lock"),
            timeout: 10,
        }
    }
    pub fn set_timeout(&mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self.to_owned()
    }
    pub fn set_path(&mut self, path: &str) -> Self {
        self.path = String::from(path);
        self.to_owned()
    }

    pub fn open(&mut self) -> Result<Self, MutexError> {
        self.state = MutexState::Open;
        self.state.sync(&self.path)?;
        Ok(self.to_owned())
    }
    pub fn lock(&mut self) -> Result<Self, MutexError> {
        self.state = MutexState::Locked;
        self.state.sync(&self.path)?;
        Ok(self.to_owned())
    }

    // locks or opens withot modifying file
    pub fn local_lock(&mut self) -> Result<Self, MutexError> {
        self.state = MutexState::Locked;
        Ok(self.to_owned())
    }
    pub fn lock_open(&mut self) -> Result<Self, MutexError> {
        self.state = MutexState::Open;
        Ok(self.to_owned())
    }

    pub fn sync(&mut self) -> Result<Self, MutexError> {
        self.state.sync(&self.path)?;
        Ok(self.to_owned())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutexState {
    Locked,
    Open,
}

impl MutexState {
    pub fn from_path(path: &str) -> Result<Self, MutexError> {
        match std::fs::read(path) {
            Err(fs_error) => Err(MutexError::File(fs_error)),
            Ok(file) => {
                let string = match String::from_utf8(file) {
                    Err(fs_error) => return Err(MutexError::Generic(fs_error.to_string())),
                    Ok(file) => file,
                };
                Ok(MutexState::from_string(&string)?)
            }
        }
    }
    pub fn is_locked(&self) -> bool {
        match self {
            MutexState::Locked => true,
            MutexState::Open => false,
        }
    }

    pub fn is_open(&self) -> bool {
        !self.is_locked()
    }

    // updates mutex file with current state
    pub fn sync(&self, path: &str) -> Result<(), MutexError> {
        std::fs::write(path, self.to_string()).unwrap();

        if let Err(fs_error) = std::fs::write(path, self.to_string()) {
            return Err(MutexError::File(fs_error));
        };
        Ok(())
    }

    pub fn from_string(input: &str) -> Result<Self, MutexError> {
        // Remove all non-alphanumeric characters using regex
        let regex = Regex::new(r#"[^a-zA-Z0-9]+"#).unwrap();
        let input: String = regex.replace_all(input, "").into();
        match input.as_str() {
            "Locked" => Ok(MutexState::Locked),
            "Open" => Ok(MutexState::Open),
            _ => Err(MutexError::State(MutexExceptions::InvalidState)),
        }
    }
}
#[derive(Debug)]
pub enum MutexError {
    // file system errors
    File(std::io::Error),
    // mutex state specific errors
    State(MutexExceptions),
    // generic (stuff like utf8 or unhandled errors
    Generic(String),
}
#[derive(Debug)]
pub enum MutexExceptions {
    // tried to lock mutex, but mutex was already locked
    AlreadyLocked,
    // Could not read mutex state (invalid file)
    InvalidState,
}

use std::fmt;

impl fmt::Display for MutexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MutexError::File(fs_error) => write!(
                f,
                "Error occurred while manipulating mutex file: {}",
                fs_error
            ),
            MutexError::State(muterror) => {
                write!(f, "Error occurred with runtime mutex: {:#?}", muterror)
            }

            MutexError::Generic(error) => {
                write!(f, "{}", error)
            }
        }
    }
}

impl fmt::Display for MutexState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = match self {
            MutexState::Locked => "Locked",
            MutexState::Open => "Open",
        };

        write!(f, "{state}")
    }
}
