use std::{
    fmt::Display,
    ops::{AddAssign, MulAssign},
    str::FromStr,
};

use thiserror::Error;

const EMPTY_CELL: u8 = b'.';

#[derive(Debug, Clone, Copy, Default)]
pub struct Run {
    pub len: usize,
    pub item: u8,
}

impl Run {
    pub fn pad(len: usize) -> Option<Self> {
        (len != 0).then(|| Self {
            len,
            item: EMPTY_CELL,
        })
    }

    fn display_len(&self) -> usize {
        match self.len {
            0 => 0,
            1 => 1,
            len => len.ilog10() as usize + 2,
        }
    }
}

#[test]
fn test_run_char_ct() {
    let run = Run {
        len: 0,
        item: b'\0',
    };
    assert_eq!(run.display_len(), 0);

    let run = Run {
        len: 1,
        item: b'\0',
    };
    assert_eq!(run.display_len(), 1);

    let run = Run {
        len: 2,
        item: b'\0',
    };
    assert_eq!(run.display_len(), 2);

    let run = Run {
        len: 9,
        item: b'\0',
    };
    assert_eq!(run.display_len(), 2);

    let run = Run {
        len: 10,
        item: b'\0',
    };
    assert_eq!(run.display_len(), 3);

    let run = Run {
        len: 99,
        item: b'\0',
    };
    assert_eq!(run.display_len(), 3);

    let run = Run {
        len: 100,
        item: b'\0',
    };
    assert_eq!(run.display_len(), 4);
}

impl Display for Run {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Run { len, item } = *self;
        match len {
            0 => {
                debug_assert!(false, "zero length run found in rle");
                Ok(())
            }
            1 => write!(f, "{}", item as char),
            len => write!(f, "{len}{}", item as char),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct RleLine {
    pub runs: Vec<Run>,
}

impl RleLine {
    pub fn pad(len: usize) -> Self {
        Self {
            runs: Vec::from_iter(Run::pad(len)),
        }
    }

    pub fn len(&self) -> usize {
        self.runs.iter().map(|run| run.len).sum::<usize>()
    }
}

impl AddAssign for RleLine {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl AddAssign<&Self> for RleLine {
    fn add_assign(&mut self, rhs: &Self) {
        let Some(last) = self.runs.last().copied() else {
            self.runs.clone_from(&rhs.runs);
            return;
        };
        let Some(mut first) = rhs.runs.first().copied() else {
            return;
        };
        if last.item == first.item {
            self.runs.pop();
            first.len += last.len;
        }
        self.runs.push(first);
        self.runs.extend_from_slice(&rhs.runs[1..]);
    }
}

#[derive(Debug, Clone, Default)]
pub struct Rle {
    lines: Vec<RleLine>,
}

impl Rle {
    pub fn new_indent(w: usize, h: usize) -> Self {
        Self {
            lines: vec![RleLine::pad(w); h],
        }
    }

    pub fn stack(&mut self, rhs: &Self) {
        self.lines.extend_from_slice(&rhs.lines);
    }

    pub fn clear(&mut self) {
        self.lines.clear();
    }

    pub fn width(&self) -> usize {
        self.lines
            .iter()
            .map(|line| line.len())
            .max()
            .unwrap_or_default()
    }

    pub fn height(&self) -> usize {
        self.lines.len()
    }

    fn build_from(&mut self, s: &str, w: usize, h: usize) {
        let mut line = RleLine::default();
        let mut pad_len = w;
        let mut run_len = 0;
        for c in s.chars().filter(|c| !c.is_whitespace()) {
            if let Some(digit) = c.to_digit(10) {
                run_len *= 10;
                run_len += digit as usize;
                continue;
            }
            if "$!".contains(c) {
                if pad_len != 0 {
                    line.runs.push(Run {
                        len: pad_len,
                        item: EMPTY_CELL,
                    });
                }
                pad_len = w;
                self.lines.push(line);
                line = RleLine::default();
                for _ in 1..run_len {
                    self.lines.push(RleLine::default());
                }
            } else {
                let len = run_len.max(1);
                pad_len -= len;
                line.runs.push(Run { len, item: c as u8 });
            }
            run_len = 0;
        }
        self.pad_bottom_to(w, h);
    }

    fn pad_bottom_to(&mut self, w: usize, h: usize) {
        let pad_amount = h - self.lines.len();
        if pad_amount == 0 {
            return;
        }
        let pad_line = RleLine::pad(w);
        for _ in 1..pad_amount {
            self.lines.push(pad_line.clone());
        }
        self.lines.push(pad_line);
    }
}

impl AddAssign for Rle {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

impl AddAssign<&Self> for Rle {
    fn add_assign(&mut self, rhs: &Self) {
        assert_eq!(self.lines.len(), rhs.lines.len());
        for (a, b) in self.lines.iter_mut().zip(&rhs.lines) {
            *a += b;
        }
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
        let w = w.parse::<usize>().map_err(|_| RleErr::BadWidth)?;

        let s = s.trim_start_matches("y = ");
        let (h, s) = s.split_once(", ").ok_or(RleErr::NoHeight)?;
        let h = h.parse::<usize>().map_err(|_| RleErr::BadHeight)?;

        let mut out = Rle::default();
        if h == 0 {
            return Ok(out);
        }

        let s = s.trim_start_matches("rule = Flow6\n");
        if w != 0 {
            out.build_from(s, w, h);
        } else {
            out.lines = vec![RleLine::default(); h];
        }

        Ok(out)
    }
}

impl Display for Rle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "x = {}, y = {}, rule = Flow6",
            self.width(),
            self.height(),
        )?;
        const MAX_LINE_WIDTH: usize = 70;
        let mut line_chars = MAX_LINE_WIDTH;

        let mut write = |run: &Run| -> std::fmt::Result {
            let len = run.display_len();
            if len >= line_chars {
                writeln!(f)?;
                line_chars = MAX_LINE_WIDTH;
            }
            line_chars -= len;
            write!(f, "{run}")
        };

        let last_line = self.lines.len() - 1;
        let mut newlines = 1;
        for (i, line) in self.lines.iter().enumerate() {
            if line.runs.is_empty() {
                newlines += 1;
                continue;
            }
            for run in &line.runs[0..line.runs.len() - 1] {
                write(run)?;
            }
            if let Some(run) = line.runs.get(line.runs.len() - 1) {
                if run.item != EMPTY_CELL {
                    write(run)?;
                }
            }
            let newline = if i == last_line { b'!' } else { b'$' };
            write(&Run {
                len: newlines,
                item: newline,
            })?;
            newlines = 1;
        }
        writeln!(f)
    }
}

impl MulAssign<usize> for Rle {
    fn mul_assign(&mut self, rhs: usize) {
        if rhs == 0 {
            self.lines.clear();
            return;
        }
        if rhs == 1 {
            return;
        }
        let clone = self.clone();
        for _ in 1..rhs {
            *self += &clone;
        }
    }
}
