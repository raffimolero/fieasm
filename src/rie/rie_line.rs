use super::{
    arm_cmd::BadArmCmd,
    header::HeaderFormat,
    register_cmd::{BadRegisterCmd, RegisterCmd},
    tm_cmd::TMCmd,
};
use crate::{
    helpers::{get_tokens, next_token},
    rie::arm_cmd::ArmCmd,
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RieLineErr {
    #[error("There was a tab, indicating a real line, but no state was specified.")]
    NoState,

    #[error(
        "Could not parse the 'state'.\n\
        States must be non-negative integers, but I found {0:?}."
    )]
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
    BadRegisterCommand(u32, bool, usize, BadRegisterCmd),

    #[error("Invalid command at State {0} with Arg {1} for Construction Arm: {2}")]
    BadArmCommand(u32, bool, BadArmCmd),

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
        let mut instructions = vec![format!("Goto {:>state_digits$}", cmd.goto)];

        #[derive(Debug, PartialEq)]
        enum ReadSource {
            Constant(bool),
            Register(usize),
            Arm,
        }

        // which register this command reads from, if any
        let mut read_from = cmd.read.map(ReadSource::Constant);

        // list every register command
        for (i, &cmd) in cmd
            .register_cmds
            .iter()
            .enumerate()
            .filter(|&(_i, cmd)| *cmd != RegisterCmd::Noop)
        {
            if cmd == RegisterCmd::Read {
                debug_assert_eq!(
                    read_from.replace(ReadSource::Register(i)),
                    None,
                    "A TMCmd somehow ended up having multiple reads."
                );
                continue;
            }
            instructions.push(format!("{cmd:?} Register {i}"));
        }

        if cmd.arm_cmd.0[ArmCmd::READ] {
            debug_assert_eq!(
                read_from.replace(ReadSource::Arm),
                None,
                "A TMCmd somehow ended up having multiple reads."
            );
        }

        // list which source to read from for the next command
        if let Some(source) = read_from {
            let source = match source {
                ReadSource::Constant(val) => format!("Constant {val}"),
                ReadSource::Register(id) => format!("Register {id}"),
                ReadSource::Arm => "Construction Arm".to_string(),
            };
            instructions.push(format!("Read from {source}"));
        }

        // compile all instructions in the list
        format!(
            "State {state:>state_digits$} | {arg:<5} = {{ {} }}",
            instructions.join(" | ")
        )
    }

    pub fn parse(line: &str, format: HeaderFormat) -> Result<Self, RieLineErr> {
        use RieLineErr::*;

        let mut tokens = get_tokens(line);
        let state = next_token(&mut tokens, |_, token| BadState(token))?.ok_or(NoState)?;
        let arg = next_token(&mut tokens, |_, token| BadArg(token, state))?.ok_or(NoArg(state))?;
        let goto = next_token(&mut tokens, |_, token| BadJump(token, state, arg))?.unwrap_or(state);
        let read = next_token(&mut tokens, |_, token| BadRead(token, state, arg))?;

        let mut register_cmds = vec![];
        for (i, token) in (0..format.register_count).zip(&mut tokens) {
            register_cmds.push(
                token
                    .parse::<RegisterCmd>()
                    .map_err(|e| BadRegisterCommand(state, arg, i, e))?,
            );
        }

        let arm_cmd: ArmCmd = format
            .has_arm
            .then(|| next_token(&mut tokens, |e, _| BadArmCommand(state, arg, e)))
            .transpose()?
            .flatten()
            .unwrap_or_default();

        let read_count = register_cmds
            .iter()
            .filter(|&cmd| *cmd == RegisterCmd::Read)
            .count()
            + read.is_some() as usize
            + arm_cmd.0[ArmCmd::READ] as usize;

        if read_count > 1 {
            return Err(MultiRead(state, arg));
        }

        let cmd = TMCmd {
            goto,
            read,
            register_cmds,
            arm_cmd,
        };

        Ok(Self { state, arg, cmd })
    }
}
