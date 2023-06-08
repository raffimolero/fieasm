use std::str::FromStr;
use thiserror::Error;

use super::{Register, Rle};

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
}

impl Register<4> for RegisterCmd {
    const TOLERANCE: usize = 18;

    fn assemble(&self) -> [bool; 4] {
        use RegisterCmd::*;
        match self {
            Noop => [false; 4],
            Push => [false, true, false, false],
            Pull => [false, false, true, false],
            Flip => [false, true, true, true],
            FlipPull => [false, false, true, true],
            Read => [true, false, false, true],
        }
    }

    fn rle() -> Rle {
        "x = 29, y = 16, rule = Flow6
        .3A.2A11.A2.A$2A.3A.18A$.3A.2A.2A.2A5.A2.A2.A$3.A5.A2.5A.A2.2A.A$3.7A
        6.A.A2.8A$2A8.3A3.A.A2.A2.2A2.A$.10A.5A.A2.A2.A$10.3A2.2A.A2.A.3A2.A$
        .3A.3A8.A.A2.A.A.A$2A.A.A.A.3A.4A.2A.A2.A3.A$3.3A.3A.3A.2A.11A$3A13.A
        .A2.A2.2A$A.A.3A.3A.3A.A.A2.A$A.3A.3A.3A.3A.A2.A$4.3A.3A.3A3.A2.A$18.
        A2.A!"
            .parse()
            .unwrap()
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
