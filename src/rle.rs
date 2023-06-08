use std::{
    fmt::Display,
    ops::{AddAssign, BitOrAssign},
    str::FromStr,
};

use thiserror::Error;

#[derive(Debug, Clone, Copy, Default)]
pub struct Run {
    len: u32,
    item: char,
}

impl Display for Run {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Run { len, item } = *self;
        match len {
            0 => {
                debug_assert!(false, "zero length run found in rle");
                Ok(())
            }
            1 => write!(f, "{item}"),
            len => write!(f, "{len}{item}"),
        }
    }
}

#[derive(Debug, Default)]
pub struct RleLine {
    runs: Vec<Run>,
}

impl AddAssign for RleLine {
    fn add_assign(&mut self, mut rhs: Self) {
        let Some(last) = self.runs.last().copied() else {
           return;
        };
        let Some(first) = rhs.runs.first_mut() else {
            *self = rhs;
            return;
        };
        if last.item == first.item {
            self.runs.pop();
            first.len += last.len;
        }
        self.runs.append(&mut rhs.runs);
    }
}

#[derive(Debug, Default)]
pub struct Rle {
    lines: Vec<RleLine>,
}

impl AddAssign for Rle {
    fn add_assign(&mut self, rhs: Self) {
        assert_eq!(self.lines.len(), rhs.lines.len());
        for (a, b) in self.lines.iter_mut().zip(rhs.lines) {
            *a += b;
        }
    }
}

impl BitOrAssign for Rle {
    fn bitor_assign(&mut self, mut rhs: Self) {
        self.lines.append(&mut rhs.lines);
    }
}

#[derive(Error, Debug)]
pub enum RleErr {
    #[error("The RLE did not contain a width.")]
    NoWidth,

    #[error("The RLE contained an invalid width.")]
    BadWidth,

    #[error("The RLE did not contain a height.")]
    NoHeight,

    #[error("The RLE contained an invalid height.")]
    BadHeight,
}

impl FromStr for Rle {
    type Err = RleErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches("x = ");
        let (w, s) = s.split_once(", ").ok_or(RleErr::NoWidth)?;
        let w = w.parse::<u32>().map_err(|_| RleErr::BadWidth)?;

        let s = s.trim_start_matches("y = ");
        let (h, s) = s.split_once(", ").ok_or(RleErr::NoHeight)?;
        let h = h.parse::<u32>().map_err(|_| RleErr::BadHeight)?;

        let s = s.trim_start_matches(|c| c != '\n');
        let mut out = Rle::default();
        let mut line = RleLine::default();
        let mut pad_len = w;
        let mut run_len = 0;
        for c in s.chars().filter(|c| !c.is_whitespace()) {
            if let Some(digit) = c.to_digit(10) {
                run_len *= 10;
                run_len += digit;
                continue;
            }
            if "$!".contains(c) {
                if pad_len != 0 {
                    line.runs.push(Run {
                        len: pad_len,
                        item: '.',
                    });
                }
                pad_len = w;
                out.lines.push(line);
                line = RleLine::default();
                for _ in 1..run_len {
                    out.lines.push(RleLine::default());
                }
            } else {
                let len = run_len.max(1);
                pad_len -= len;
                line.runs.push(Run { len, item: c });
            }
            run_len = 0;
        }
        let pad = h - out.lines.len() as u32;
        for _ in 0..pad {
            out.lines.push(RleLine::default());
        }
        Ok(out)
    }
}

impl Display for Rle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "x = 0, y = 0, rule = Flow6")?;
        let last_line = self.lines.len() - 1;
        let mut newlines = 1;
        for (i, line) in self.lines.iter().enumerate() {
            if line.runs.is_empty() {
                newlines += 1;
                continue;
            }
            for run in &line.runs {
                write!(f, "{run}")?;
            }
            let newline = if i == last_line { '!' } else { '$' };
            writeln!(
                f,
                "{}",
                Run {
                    len: newlines,
                    item: newline
                }
            )?;
            newlines = 1;
        }
        Ok(())
    }
}
