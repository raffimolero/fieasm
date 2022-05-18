use super::{
    register_cmd::{BadRegisterCmd, RegisterCmd},
    tm_cmd::TMCmd,
};
use crate::helpers::{get_tokens, next_token};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum FieLineErr {
    #[error("No state specified.")]
    NoState,

    #[error("Could not parse the 'state' ({0}).")]
    BadState(String),

    #[error(
        "State {0} has no arg.\n\
        Please specify an arg of true or false."
    )]
    NoArg(u32),

    #[error(
        "Could not parse the 'arg' for State {1}.\n\
        Args must be either true or false, but I found {0:?}."
    )]
    BadArg(String, u32),

    #[error(
        "Could not parse the 'jump' for State {1} with Arg {2}.\n\
        Jumps must be non-negative integers, but I found {0:?}."
    )]
    BadJump(String, u32, bool),

    #[error(
        "Could not parse the 'read' for State {1} with Arg {2}.\n\
        Reads must be either true or false, but I found {0:?}."
    )]
    BadRead(String, u32, bool),

    #[error("Invalid command at State {0} with Arg {1} for Register {2}: {3}")]
    BadCommand(u32, bool, usize, BadRegisterCmd),

    #[error(
        "Attempt to read from multiple sources at State {0} with Arg {1}.\n\
        There must only be at most 1 read per command."
    )]
    MultiRead(u32, bool),
}

pub struct FieLine {
    pub state: u32,
    pub arg: bool,
    pub cmd: TMCmd,
}

impl FromStr for FieLine {
    type Err = FieLineErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use FieLineErr::*;

        let tokens = &mut get_tokens(s);

        let state = next_token(tokens, BadState)?.ok_or(NoState)?;
        let arg = next_token(tokens, |token| BadArg(token, state))?.ok_or(NoArg(state))?;
        let jump = next_token(tokens, |token| BadJump(token, state, arg))?.unwrap_or(state);
        let read = next_token(tokens, |token| BadRead(token, state, arg))?;

        let mut register_cmds = vec![];
        for (i, token) in tokens.enumerate() {
            register_cmds.push(
                token
                    .parse::<RegisterCmd>()
                    .map_err(|e| BadCommand(state, arg, i, e))?,
            );
        }

        // check if there are multiple reads
        if register_cmds
            .iter()
            .filter(|&cmd| *cmd == RegisterCmd::Read)
            .count()
            + read.is_some() as usize
            > 1
        {
            return Err(MultiRead(state, arg));
        }

        let cmd = TMCmd {
            jump_mask: jump ^ state,
            read,
            register_cmds,
        };

        Ok(Self { state, arg, cmd })
    }
}
