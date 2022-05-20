use super::register_cmd::RegisterCmd;
use crate::helpers::extend_vec_to;

#[derive(Debug, Clone, Default)]
pub struct TMCmd {
    pub goto: u32,
    pub read: Option<bool>,
    pub register_cmds: Vec<RegisterCmd>,
}

impl TMCmd {
    pub fn assemble(&self, state_bit_count: u32, register_count: usize) -> Vec<Vec<bool>> {
        // assemble register commands
        let mut out = self
            .register_cmds
            .iter()
            .rev()
            .map(|cmd| cmd.assemble().to_vec())
            .collect();
        extend_vec_to(&mut out, vec![false; 4], register_count);

        // assemble read
        out.push(self.read.map_or(vec![false; 2], |bit| vec![!bit, bit]));

        // assemble jump mask
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
