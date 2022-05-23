pub mod header;
pub mod register_cmd;
pub mod rie_line;
pub mod tm_cmd;

use self::{
    header::{HeaderErr, HeaderFormat},
    rie_line::{RieLine, RieLineErr},
    tm_cmd::TMCmd,
};
use crate::helpers::{
    ask_y_n, break_string, extend_vec_to, largest_bit, pause, RED, RESET, YELLOW,
};
use std::{
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RieErr {
    #[error("IO Error: {0}")]
    IO(#[from] io::Error),

    #[error("The specified flipifelse file has no header.")]
    NoHeader,

    #[error("{0}")]
    BadHeader(#[from] HeaderErr),

    #[error("Error on line {0}: {1}")]
    BadLine(usize, RieLineErr),

    #[error("Cancelled compilation.")]
    Cancelled,
}

pub struct RieProgram {
    commands: Vec<[TMCmd; 2]>,
    state_bits: u32,
    register_count: usize,
}
impl RieProgram {
    pub fn len(&self) -> usize {
        self.commands.len() * 2
    }

    pub fn assemble(&self) -> Vec<[Vec<Vec<bool>>; 2]> {
        self.commands
            .iter()
            .map(|pair| {
                pair.clone()
                    .map(|cmd| cmd.assemble(self.state_bits, self.register_count))
            })
            .collect()
    }

    pub fn rle(&self) -> String {
        let column_pairs = self.assemble();

        // create a vec of lines in the same format as a column
        let mut row = 0;
        let first_column = &column_pairs[0][false as usize];
        let mut lines = first_column
            .iter()
            .map(|segment| {
                segment
                    .iter()
                    .map(|_| {
                        let line = match row {
                            0 => String::new(),
                            1 => ".".to_owned(),
                            n => format!("{n}."),
                        };
                        row += 1;
                        line
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        // rename
        let row_count = row;
        let segment_count = first_column.len();

        const COLUMN_SPACING: usize = 3;
        const PAIR_SPACING: usize = 4;
        const ROW_SPACING: usize = 3;
        const SEGMENT_SPACING: usize = 5;
        const ROW_OFFSET: usize = 1;

        // extend each respective segment
        for pair in column_pairs.iter() {
            for (arg, column) in pair.iter().enumerate() {
                for (bits, strings) in column.iter().zip(lines.iter_mut()) {
                    for (bit, string) in bits.iter().zip(strings.iter_mut()) {
                        string.push(if *bit { 'B' } else { 'C' });

                        // TODO: do not add spacing if last column
                        let gap = if arg == 1 {
                            PAIR_SPACING
                        } else {
                            COLUMN_SPACING
                        };
                        string.push_str(&format!("{gap}.",));
                    }
                }
            }
        }

        // off by one error nesting grounds
        // sanitized dw
        // not like golly cares anyway
        let w = (column_pairs.len() * (1 + COLUMN_SPACING + 1 + PAIR_SPACING) - PAIR_SPACING)
            + ((row_count - 1) * ROW_OFFSET);
        let h = (segment_count * (SEGMENT_SPACING - ROW_SPACING)) + (row_count * (1 + ROW_SPACING))
            - SEGMENT_SPACING;

        // join the rows and segments into an RLE
        format!(
            "x = {w}, y = {h}, rule = Flow6\n{}!",
            break_string(
                &lines
                    .iter()
                    .map(|rows| rows.join(&format!("{}$", 1 + ROW_SPACING)))
                    .collect::<Vec<_>>()
                    .join(&format!("{}$", 1 + SEGMENT_SPACING)),
                69
            )
        )
    }
}

impl TryFrom<File> for RieProgram {
    type Error = RieErr;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        use RieErr::*;

        eprintln!("Compiling to IR...");

        let mut lines = BufReader::new(value)
            .lines()
            .enumerate()
            .filter_map(|(i, line)| {
                line.as_ref()
                    .map_or(false, |line| line.starts_with(|c: char| c.is_whitespace()))
                    .then(|| (i + 1, line))
            });

        // Read and validate primary headers
        let (_i, line) = lines.next().ok_or(NoHeader)?;
        let HeaderFormat { register_count } = line?.parse::<HeaderFormat>()?;

        let mut warning_size = 1 << 6;
        // let mut highest_state = 0;
        let mut commands = vec![];
        let mut add_cmd = |line: usize,
                           state: u32,
                           arg: bool,
                           tm_cmd: TMCmd|
         -> Result<(), RieErr> {
            let extension = state.max(tm_cmd.goto) as usize;

            if extension > warning_size {
                let total_cmds = 2 << largest_bit(extension);
                eprintln!(
                    "{RED}WAIT!{RESET}\n\
                    The program is trying to create at least {RED}{total_cmds} instructions.{RESET}\n\
                    This is coming from line {line}, which specifies that state {state} must goto {}.\n\
                    Are you sure you want a machine with that many instructions? I will notify you if it goes beyond another power of two.",
                    tm_cmd.goto
                );
                if !ask_y_n() {
                    return Err(Cancelled);
                }
                eprintln!("Continuing...");
                warning_size = total_cmds >> 1;
            }

            /*
            if highest_state > state {
                eprintln!("{YELLOW}Warning: Lines out of order. Line {line} should probably come earlier.{RESET}");
                pause();
            }
            highest_state = state;
            */
            extend_vec_to(
                &mut commands,
                [TMCmd::default(), TMCmd::default()],
                extension + 1,
            );
            commands[state as usize][arg as usize] = tm_cmd;

            Ok(())
        };

        for (i, line) in lines {
            let line = line?;
            if line.starts_with('\t') {
                let RieLine { state, arg, cmd } =
                    RieLine::parse(&line, register_count).map_err(|e| BadLine(i, e))?;
                add_cmd(i, state, arg, cmd)?;
            }
        }

        let state_bits = largest_bit(commands.len() - 1);
        extend_vec_to(
            &mut commands,
            [TMCmd::default(), TMCmd::default()],
            1 << state_bits,
        );

        Ok(RieProgram {
            commands,
            register_count,
            state_bits,
        })
    }
}

impl Display for RieProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}-bit State ({}-bit Demux), {} Registers:",
            self.state_bits,
            self.state_bits + 1,
            self.register_count
        )?;
        let state_digits = ((1 << self.state_bits) as f32).log10() as usize + 1;
        for (state, [cmd0, cmd1]) in self.commands.iter().enumerate() {
            writeln!(
                f,
                "{}\n{}",
                RieLine::to_string(state_digits, state as u32, false, cmd0),
                RieLine::to_string(state_digits, state as u32, true, cmd1),
            )?;
        }
        Ok(())
    }
}
