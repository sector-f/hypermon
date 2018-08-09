use virt::domain::*;
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
            VIR_DOMAIN_NOSTATE => State::NoState,
            VIR_DOMAIN_RUNNING => State::Running,
            VIR_DOMAIN_BLOCKED => State::Blocked,
            VIR_DOMAIN_PAUSED => State::Paused,
            VIR_DOMAIN_SHUTDOWN => State::Shutdown,
            VIR_DOMAIN_SHUTOFF => State::Shutoff,
            VIR_DOMAIN_CRASHED => State::Crashed,
            VIR_DOMAIN_PMSUSPENDED => State::PmSuspended,
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

