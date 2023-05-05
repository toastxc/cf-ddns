use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Mutex {
    pub state: MutexState,
    pub path: String,
}

impl Mutex {
    // creates a mutex based on a toml file
    // if mutex is locked before creation it will panic
    pub fn new(path: &str) -> Result<Self, MutexError> {
        let state = MutexState::from_path(path)?;

        if state.is_locked() {
            return Err(MutexError::State(MutexExceptions::AlreadyLocked));
        };

        Ok(Self {
            state,
            path: String::from(path),
        })
    }
    pub fn open(&mut self) -> Result<Self, MutexError> {
        self.state = MutexState::Open;
        self.state.sync(&self.path)?;
        Ok(self.to_owned())
    }
    pub fn lock(&mut self) -> Result<Self, MutexError> {
        self.state = MutexState::Locked;
        println!("TESTER: {}", self.state.to_string());
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
            Err(fs_error) => {
                return Err(MutexError::File(fs_error));
            }
            Ok(file) => {
                let string = String::from_utf8(file).unwrap();
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
    // updates mutex file with current state
    pub fn sync(&self, path: &str) -> Result<(), MutexError> {
        std::fs::write(path, self.to_string()).unwrap();

        if let Err(fs_error) = std::fs::write(path, self.to_string()) {
            return Err(MutexError::File(fs_error));
        };
        Ok(())
    }

    pub fn to_string(&self) -> String {
        match self {
            MutexState::Locked => "Locked",
            MutexState::Open => "Open",
        }
        .to_string()
    }
    pub fn from_string(input: &str) -> Result<Self, MutexError> {
        match input {
            "Locked" => Ok(MutexState::Locked),
            "Open" => Ok(MutexState::Open),
            _ => Err(MutexError::State(MutexExceptions::InvalidState)),
        }
    }
}
#[derive(Debug)]
pub enum MutexError {
    File(std::io::Error),
    State(MutexExceptions),
}
#[derive(Debug)]
pub enum MutexExceptions {
    // tried to lock mutex, mutex was already locked
    AlreadyLocked,
    // Could not read mutex state (invalid file, typo, etc)
    InvalidState,
}

use std::fmt;

impl fmt::Display for MutexError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MutexError::File(fs_error) => write!(
                f,
                "Error occurred while manipulating lock file\n{}",
                fs_error
            ),
            MutexError::State(MutexExceptions::AlreadyLocked) => write!(
                f,
                "Error occurred while manipulating mutex: Mutex_Already_Locked"
            ),
            MutexError::State(MutexExceptions::InvalidState) => write!(
                f,
                "Error occurred while manipulating mutex file: Invalid_Mutex_State"
            ),
        }
    }
}
