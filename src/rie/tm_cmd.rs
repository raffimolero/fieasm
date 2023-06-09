use super::{arm_cmd::ArmCmd, header::HeaderFormat, register_cmd::RegisterCmd, Register};
use crate::helpers::extend_vec_to;

#[derive(Debug, Clone, Default)]
pub struct TMCmd {
    pub goto: usize,
    pub read: Option<bool>,
    pub register_cmds: Vec<RegisterCmd>,
    pub arm_cmd: ArmCmd,
}

impl TMCmd {
    pub fn assemble(&self, state_bit_count: usize, format: HeaderFormat) -> Vec<Vec<bool>> {
        let mut out = vec![];

        // assemble arm commands
        if format.has_arm {
            out.push(self.arm_cmd.assemble().to_owned().to_vec());
        }

        extend_vec_to(
            &mut out,
            vec![false; 4],
            format.has_arm as usize + format.register_count - self.register_cmds.len(),
        );
        // assemble register commands
        out.extend(
            self.register_cmds
                .iter()
                .rev()
                .map(|cmd| cmd.assemble().to_vec()),
        );

        // assemble read
        out.push(self.read.map_or(vec![false; 2], |bit| vec![!bit, bit]));

        // assemble goto
        let mut mask = self.goto;
        let mut goto_bits = vec![];
        for _ in 0..state_bit_count {
            goto_bits.push(mask & 1 == 1);
            mask >>= 1;
        }
        out.push(goto_bits);

        out
    }
}

// If you're looking for the FromStr implementation, go to rie_line.
// It also has to_string, but because I have to pass in the number of decimal digits that State can have, it cannot be Display.
