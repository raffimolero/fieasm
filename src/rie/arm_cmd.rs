use std::str::FromStr;
use thiserror::Error;

use super::{Register, Rle};

#[derive(Error, Debug)]
#[error(
    "{0} is not an arm command.\n\
    Commands must follow the format {} as the base, and remove unneeded instructions.",
    ArmCmd::FORMAT
)]
pub struct BadArmCmd(String);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct ArmCmd(pub [bool; 4]);

impl ArmCmd {
    pub const PUSH: usize = 0;
    pub const FLIP_RETRACT: usize = 1;
    pub const BEND: usize = 2; // downgraded into a flip retract if push is active
    pub const READ: usize = 3;
    pub const FORMAT: &'static str = "<>C?";
}

impl Register<4> for ArmCmd {
    const TOLERANCE: usize = 27;

    fn assemble(&self) -> [bool; 4] {
        self.0
    }

    fn rle() -> Rle {
        "x = 40, y = 16, rule = Flow6
        3A.3A$A.3A.13A.13A5.2A$3A.A13.A.A11.A5.A$4.A13.A.7A.5A2.4A$3.2A.3A9.A
        7.A.A6.A.A$7A.3A7.A5.E.A.A4.5A$6.3A.A7.A3.2E2.A.A3.2A.A.A$10.A5.A.4AE
        .E.A.8A.A$9.2A.3A.A.2A2.AE2.A6.2A2.A$13A.3A2.A.2A.E.A4.2A.A.3A$12.3A.
        7A3.A4.A.2A.3A$.3A.3A.3A4.2A.A2.5A.5A4.A$.A.A.A.A.A.A.3A.A.A8.A.3A4.A
        $2A.3A.3A.3A.3A.A8.A2.A5.A$17.A.19A$28.A2.A!"
            .parse()
            .unwrap()
    }
}

impl FromStr for ArmCmd {
    type Err = BadArmCmd;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut this = Self::default();
        let mut iter = s.chars().peekable();
        for (i, c) in Self::FORMAT.chars().enumerate() {
            if iter.next_if_eq(&c).is_some() {
                this.0[i] = true;
            }
        }
        iter.next()
            .is_none()
            .then_some(this)
            .ok_or_else(|| BadArmCmd(s.to_owned()))
    }
}
