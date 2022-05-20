use super::{
    register_cmd::{BadRegisterCmd, RegisterCmd},
    tm_cmd::TMCmd,
};
use crate::helpers::{get_tokens, next_token};
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RieLineErr {
    #[error("There was a tab, indicating a real line, but no state was specified.")]
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

pub struct RieLine {
    pub state: u32,
    pub arg: bool,
    pub cmd: TMCmd,
}

impl RieLine {
    pub fn to_string(state_digits: usize, state: u32, arg: bool, cmd: &TMCmd) -> String {
        let mut instructions = vec![];

        // list goto address
        if cmd.goto != state {
            instructions.push(format!("Goto {}", cmd.goto));
        }

        // which register this command reads from, if any
        let mut read_register_id = None;

        // list every register command
        for (i, &cmd) in cmd
            .register_cmds
            .iter()
            .filter(|&cmd| *cmd != RegisterCmd::Noop)
            .enumerate()
        {
            if cmd == RegisterCmd::Read {
                debug_assert_eq!(
                    read_register_id, None,
                    "A TMCmd somehow ended up having multiple reads."
                );
                read_register_id = Some(i);
                continue;
            }
            instructions.push(format!("{cmd:?} Register {i}"));
        }

        // list which source to read from for the next command
        if let Some(read) = cmd
            .read
            .map(|x| x.to_string())
            .or_else(|| read_register_id.map(|x| format!("Register {x}")))
        {
            instructions.push(format!("Read {read}"));
        }

        // compile all instructions in the list
        format!(
            "State {state:>state_digits$} | {arg:<5} = {{ {} }}",
            instructions.join(" | ")
        )
    }

    pub fn parse(line: &str, register_count: usize) -> Result<Self, RieLineErr> {
        use RieLineErr::*;

        let tokens = &mut get_tokens(line);
        let state = next_token(tokens, BadState)?.ok_or(NoState)?;
        let arg = next_token(tokens, |token| BadArg(token, state))?.ok_or(NoArg(state))?;
        let goto = next_token(tokens, |token| BadJump(token, state, arg))?.unwrap_or(state);
        let mut read = next_token(tokens, |token| BadRead(token, state, arg))?;

        let mut register_cmds = vec![];
        for (i, token) in (0..register_count).zip(tokens) {
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
            goto,
            read,
            register_cmds,
        };

        Ok(Self { state, arg, cmd })
    }
}
