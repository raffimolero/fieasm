use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
#[error(
    "{0} is not a register command.\n\
    Valid commands: {:?}",
    RegisterCmd::VALID_COMMANDS.map(|(token, _cmd)| token),
)]
pub struct BadRegisterCmd(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RegisterCmd {
    Noop,
    Push,
    Pull,
    Flip,
    FlipPull,
    Read,
}

impl RegisterCmd {
    pub(crate) const VALID_COMMANDS: [(&'static str, Self); 5] = [
        (">", Self::Push),
        ("<", Self::Pull),
        ("%", Self::Flip),
        ("%<", Self::FlipPull),
        ("?", Self::Read),
    ];

    pub fn assemble(&self) -> [bool; 4] {
        use RegisterCmd::*;
        match self {
            Noop => [false; 4],
            Push => [false, false, true, false],
            Pull => [false, true, false, false],
            Flip => [true, true, true, false],
            FlipPull => [true, true, false, false],
            Read => [true, false, false, true],
        }
    }
}

impl Default for RegisterCmd {
    fn default() -> Self {
        Self::Noop
    }
}

impl FromStr for RegisterCmd {
    type Err = BadRegisterCmd;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use RegisterCmd::*;
        Self::VALID_COMMANDS
            .iter()
            .find_map(|&(token, command)| (s == token).then(|| command))
            .or_else(|| s.is_empty().then(|| Noop))
            .ok_or_else(|| BadRegisterCmd(s.to_owned()))
    }
}
