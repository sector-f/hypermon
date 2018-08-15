use std::fmt;

#[derive(Serialize)]
pub enum State {
    NoState,
    Running,
    Blocked,
    Paused,
    Shutdown,
    Shutoff,
    Crashed,
    PmSuspended,
    Unknown,
}

impl State {
    pub fn new(s: u32) -> Self {
        match s {
            0 => State::NoState,
            1 => State::Running,
            2 => State::Blocked,
            3 => State::Paused,
            4 => State::Shutdown,
            5 => State::Shutoff,
            6 => State::Crashed,
            7 => State::PmSuspended,
            _ => State::Unknown,
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            State::NoState => write!(f, "none"),
            State::Running => write!(f, "running"),
            State::Blocked => write!(f, "blocked"),
            State::Paused => write!(f, "paused"),
            State::Shutdown => write!(f, "shutdown"),
            State::Shutoff => write!(f, "shutoff"),
            State::Crashed => write!(f, "crashed"),
            State::PmSuspended => write!(f, "suspended"),
            State::Unknown => write!(f, "unknown"),
        }
    }
}

