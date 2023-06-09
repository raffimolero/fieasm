pub mod arm_cmd;
pub mod header;
pub mod register_cmd;
pub mod rie_line;
pub mod tm_cmd;

use self::{
    arm_cmd::ArmCmd,
    header::{HeaderErr, HeaderFormat},
    rie_line::{RieLine, RieLineErr},
    tm_cmd::TMCmd,
};
use crate::{
    helpers::{ask_y_n, break_string, extend_vec_to, largest_bit, RESET, YELLOW},
    rle::Rle,
};
use std::{
    arch::x86_64::_SIDD_BIT_MASK,
    fmt::Display,
    fs::File,
    io::{self, BufRead, BufReader},
    ops::{AddAssign, BitOrAssign},
    str::FromStr,
};

use thiserror::Error;

/// rust does not support using associated consts as the length of an array
pub trait Register<const BITS: usize>: FromStr {
    /// space between the first rom bit and the read columns
    const TOLERANCE: usize;

    fn assemble(&self) -> [bool; BITS];
    fn rle() -> Rle;
}

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
    format: HeaderFormat,
    state_bits: usize,
    // TODO: add support for putting data in
    // data: Vec<Vec<bool>>,
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
                    .map(|cmd| cmd.assemble(self.state_bits, self.format))
            })
            .collect()
    }

    fn build_rom(rom_column_bits: usize, rom_data: Vec<[Vec<Vec<bool>>; 2]>) -> Rle {
        // // actual rom size
        // let rom_height = rom_column_bits * 4;
        let rom_true = "x = 5, y = 4, rule = Flow6
                .A$5A$.A.B$.3A!"
            .parse::<Rle>()
            .unwrap();
        let rom_false = "x = 5, y = 4, rule = Flow6
                .A$5A$.A.C$.3A!"
            .parse::<Rle>()
            .unwrap();
        let rom_pad = "x = 1, y = 4, rule = Flow6
                !"
        .parse::<Rle>()
        .unwrap();
        let rom_row = "x = 1, y = 4, rule = Flow6
                $A!"
        .parse::<Rle>()
        .unwrap();

        let mut out = Rle::default();

        let mut indent = Rle::new_indent(0, 4);
        for i in 0..rom_column_bits {
            out.stack(&indent);
            indent += &rom_pad;
        }
        let mut column = Rle::default();
        // data (-> pairs ->) columns (-> registers ->) bits
        for column_bits in rom_data.iter().flatten().rev() {
            column.clear();
            for tile in column_bits.iter().flatten().copied() {
                column.stack(if tile { &rom_true } else { &rom_false });
            }
            out += &column;
        }

        out
    }

    fn build_demux(len: usize, state_bits: usize, rom_column_bits: usize) -> Rle {
        // max command id that the demux needs to support
        let demux_max = len * 2;
        let demux_height = (state_bits + 1) * 5 + 3;
        let demux_width = len * 5;
        // physical location of the left edge of the demux
        let demux_left_edge = rom_column_bits - 2;
        // // just beyond the right edge of the demux
        // let demux_right_edge = demux_left_edge + demux_width;

        let demux_true = "x = 5, y = 5, rule = Flow6
            .A.A$5A$2A.A$.2A.A$2.C2A!"
            .parse::<Rle>()
            .unwrap();
        let demux_false = "x = 5, y = 5, rule = Flow6
            .A.A$5A$2A.A$.2A.A$2.B2A!"
            .parse::<Rle>()
            .unwrap();
        let demux_bottom = "x = 5, y = 3, rule = Flow6
            3.A$2.2A$5A!"
            .parse::<Rle>()
            .unwrap();

        let mut out = Rle::new_indent(demux_left_edge, demux_height);

        let mut column = Rle::default();
        for address in (0..demux_max).rev() {
            column.clear();
            for b in (0..state_bits + 1).rev() {
                column.stack(if ((address >> b) & 1) != 0 {
                    &demux_true
                } else {
                    &demux_false
                });
            }
            column.stack(&demux_bottom);
            out += &column;
        }

        // let demux_catch = "x = 7, y = 5, rule = Flow6
        //     .3A.A$A2.3A$.A2.A$2.5A$4.A!"
        //     .parse::<Rle>()
        //     .unwrap();
        // let demux_catch_bend = "x = 2, y = 5, rule = Flow6
        //     A$A$A$A!"
        //     .parse::<Rle>()
        //     .unwrap();
        // let demux_column = "x = 3, y = 5, rule = Flow6
        //     .A$.A$.A$.A$.A!"
        //     .parse::<Rle>()
        //     .unwrap();
        // let demux_row = "x = 2, y = 5, rule = Flow6
        //     3$2A!"
        //     .parse::<Rle>()
        //     .unwrap();
        // let activation = "x = 22, y = 2, rule = Flow6
        //     A.A2.EBA$3A!"
        //     .parse::<Rle>()
        //     .unwrap();
        // let demux_catch_final = "x = 7, y = 3, rule = Flow6
        //     .3A.A$.A.3A$.A2.A!"
        //     .parse::<Rle>()
        //     .unwrap();

        out
    }

    pub fn rle(&self) -> String {
        let rom_data = self.assemble();
        // println!("{TEST:?}");
        // println!("{TEST}");

        // how many commands there are
        let len = self.commands.len();

        // how many bits per rom column
        let rom_column_bits = self.format.column_height(self.state_bits);

        let mut machine = Rle::default();
        machine.stack(&Self::build_rom(rom_column_bits, rom_data));
        machine.stack(&Self::build_demux(len, self.state_bits, rom_column_bits));

        machine.to_string()
        // let machine_height = rom_height + demux_height + 5;
        // let mut lines: Vec<String> = Vec::with_capacity(machine_height);

        // // distance from end of 'read false' rom bit to 'read true' column
        // let mut read_distance = 6 + self.state_bits as usize * 3;

        // // create a vec of lines in the same format as a column
        // let mut row = 0;
        // let first_column = &rom_data[0][false as usize];

        //     let mut lines = first_column
        //         .iter()
        //         .map(|segment| {
        //             segment
        //                 .iter()
        //                 .map(|_| {
        //                     let line = match row {
        //                         0 => String::new(),
        //                         1 => ".".to_owned(),
        //                         n => format!("{n}."),
        //                     };
        //                     row += 1;
        //                     line
        //                 })
        //                 .collect::<Vec<_>>()
        //         })
        //         .collect::<Vec<_>>();
        //     // rename
        //     let row_count = row;
        //     let segment_count = first_column.len();

        //     const COLUMN_SPACING: usize = 3;
        //     const PAIR_SPACING: usize = 4;
        //     const ROW_SPACING: usize = 3;
        //     const SEGMENT_SPACING: usize = 5;
        //     const ROW_OFFSET: usize = 1;

        //     // extend each respective segment
        //     for pair in rom_data.iter() {
        //         for (arg, column) in pair.iter().enumerate() {
        //             for (bits, strings) in column.iter().zip(lines.iter_mut()) {
        //                 for (bit, string) in bits.iter().zip(strings.iter_mut()) {
        //                     string.push(if *bit { 'B' } else { 'C' });

        //                     // TODO: do not add spacing if last column
        //                     let gap = if arg == 1 {
        //                         PAIR_SPACING
        //                     } else {
        //                         COLUMN_SPACING
        //                     };
        //                     string.push_str(&format!("{gap}.",));
        //                 }
        //             }
        //         }
        //     }

        //     // off by one error nesting grounds
        //     // sanitized dw
        //     // not like golly cares anyway
        //     let w = (rom_data.len() * (1 + COLUMN_SPACING + 1 + PAIR_SPACING) - PAIR_SPACING)
        //         + ((row_count - 1) * ROW_OFFSET);
        //     let h = (segment_count * (SEGMENT_SPACING - ROW_SPACING)) + (row_count * (1 + ROW_SPACING))
        //         - SEGMENT_SPACING;

        //     // join the rows and segments into an RLE
        //     format!(
        //         "x = {w}, y = {h}, rule = Flow6\n{}!",
        //         break_string(
        //             &lines
        //                 .iter()
        //                 .map(|rows| rows.join(&format!("{}$", 1 + ROW_SPACING)))
        //                 .collect::<Vec<_>>()
        //                 .join(&format!("{}$", 1 + SEGMENT_SPACING)),
        //             69
        //         )
        //     )
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
        let format = line?.parse::<HeaderFormat>()?;

        let mut warning_size = 1 << 6;
        // let mut highest_state = 0;
        let mut commands = vec![];
        let mut add_cmd = |line: usize,
                           state: usize,
                           arg: bool,
                           tm_cmd: TMCmd|
         -> Result<(), RieErr> {
            let extension = state.max(tm_cmd.goto) as usize;

            if extension > warning_size {
                let total_cmds = 2 << largest_bit(extension);
                eprintln!(
                    "{YELLOW}WAIT!{RESET}\n\
                    The program is trying to create at least {YELLOW}{total_cmds} instructions.{RESET}\n\
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
                    RieLine::parse(&line, format).map_err(|e| BadLine(i, e))?;
                add_cmd(i, state, arg, cmd)?;
            }
            // else if line.starts_with("DATA") {
            //     line.trim_start_matches("DATA")
            // }
        }

        // extend_vec_to(
        //     &mut commands,
        //     [TMCmd::default(), TMCmd::default()],
        //     1 << state_bits,
        // );
        let state_bits = largest_bit(commands.len() - 1);

        Ok(RieProgram {
            commands,
            format,
            state_bits,
            // data,
        })
    }
}

impl Display for RieProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}-bit State ({}-bit Demux), {} Registers{}:",
            self.state_bits,
            self.state_bits + 1,
            self.format.register_count,
            if self.format.has_arm { " + Arm" } else { "" },
        )?;
        let state_digits = ((1 << self.state_bits) as f32).log10() as usize + 1;
        for (state, [cmd0, cmd1]) in self.commands.iter().enumerate() {
            writeln!(
                f,
                "{}\n{}",
                RieLine::to_string(state_digits, state, false, cmd0),
                RieLine::to_string(state_digits, state, true, cmd1),
            )?;
        }
        Ok(())
    }
}
