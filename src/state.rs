use std::fmt;

use crate::{data::Data};

#[derive(Debug, Copy, Clone)]
pub enum CurrentState {
    FS,
    Raid,
    NIC,
    Networking,
    IDEV,
    PDEV,
    Config,
}

impl fmt::Display for CurrentState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl CurrentState {
    pub fn next(&self) -> Self {
        use CurrentState::*;
        match *self {
            FS => Raid,
            Raid => NIC,
            NIC => Networking,
            Networking => IDEV,
            IDEV => PDEV, 
            PDEV => Config,
            Config => Config,
        }
    }

    pub fn prev(&self) -> Self {
        use CurrentState::*;
        match *self {
            FS => FS,
            Raid => FS,
            NIC => Raid,
            Networking => NIC,
            IDEV => Networking,
            PDEV => IDEV,
            Config => PDEV,
        }
    }
}

pub enum Move {
    Previous,
    Next,
}

#[derive(Clone)]
pub struct GlobalState {
    pub data: Data,
    pub current_state: CurrentState,
}