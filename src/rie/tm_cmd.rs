use super::register_cmd::RegisterCmd;
use crate::helpers::extend_vec_to;
use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct TMCmd {
    pub jump: u32,
    pub read: Option<bool>,
    pub register_cmds: Vec<RegisterCmd>,
}

impl TMCmd {
    pub fn assemble(&self, state_bit_count: u32, register_count: usize) -> Vec<Vec<bool>> {
        // assemble register commands
        let mut out = self
            .register_cmds
            .iter()
            .map(|cmd| cmd.assemble().to_vec())
            .collect();
        extend_vec_to(&mut out, vec![false; 4], register_count);

        // assemble read
        out.push(self.read.map_or(vec![false; 2], |bit| vec![!bit, bit]));

        // assemble jump mask
        let mut mask = self.jump;
        let mut jump_bits = vec![];
        for _ in 0..state_bit_count {
            jump_bits.push(mask & 1 == 1);
            mask >>= 1;
        }
        out.push(jump_bits);

        out
    }
}

// If you're looking for the FromStr implementation, go to rie_line.

impl Display for TMCmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut instructions = vec![];

        // which register this command reads from, if any
        let mut read_register_id = None;

        // list every register command
        for (i, &cmd) in self
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

        // list jump mask
        if self.jump != 0 {
            instructions.push(format!("Goto {}", self.jump));
        }

        // list which source to read from for the next command
        if let Some(read) = self
            .read
            .map(|x| x.to_string())
            .or_else(|| read_register_id.map(|x| format!("Register {x}")))
        {
            instructions.push(format!("Read {read}"));
        }

        // compile all instructions in the list
        write!(f, "{{ {} }}", instructions.join(" | "))
    }
}
