use std::{
    env,
    fs::File,
    io::{stdin, stdout, Write},
    iter::repeat,
    path::PathBuf,
    str::FromStr,
};

pub const RED: &str = "\x1b[0;31;1m";
pub const YELLOW: &str = "\x1b[0;33;1m";
pub const GREEN: &str = "\x1b[0;32;1m";
pub const RESET: &str = "\x1b[0m";

pub fn find_file(filename: &str, exec_dir: Option<String>) -> Option<File> {
    eprintln!("Searching for {filename}.");

    macro_rules! try_dir {
        ($dir:expr, $variant:tt, $notfound:expr) => {
            if let $variant(dir) = $dir {
                let mut dir = PathBuf::from(dir);
                eprintln!("Searching in {dir:?}...");
                dir.push(&filename);
                if dir.is_file() {
                    eprintln!("Found.");
                    return Some(File::open(filename).expect("Could not open file."));
                } else {
                    eprintln!("Not in {dir:?}.");
                }
            } else {
                eprintln!($notfound);
            }
        };
    }

    try_dir!(env::current_dir(), Ok, "Current dir not found.");
    try_dir!(exec_dir, Some, "Executable dir not found.");

    eprintln!("File not found.");
    None
}

pub fn get_tokens(line: &str) -> impl Iterator<Item = &str> {
    debug_assert!(
        line.starts_with(|c: char| c.is_whitespace()),
        "Tried to parse a comment as a line of code."
    );
    line.split('\t').skip(1)
}

pub fn next_token<'a, 'b, T: FromStr, E>(
    token_iter: &'a mut impl Iterator<Item = &'b str>,
    bad_token: impl Fn(T::Err, String) -> E,
) -> Result<Option<T>, E> {
    token_iter
        .next()
        .filter(|token| !token.is_empty())
        .map_or(Ok(None), |token| {
            token
                .trim()
                .parse()
                .map(Some)
                .map_err(|e| bad_token(e, token.to_owned()))
        })
}

pub fn extend_vec_to<T: Clone>(vec: &mut Vec<T>, item: T, len: usize) -> usize {
    let needed = len.saturating_sub(vec.len());
    vec.extend(repeat(item).take(needed));
    needed
}

pub fn break_string(string: &str, max_line_length: usize) -> String {
    let mut cycle = (0..max_line_length).cycle().skip(1);
    string
        .split_inclusive(|_| cycle.next().unwrap() == 0)
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn largest_bit(n: usize) -> u32 {
    usize::BITS - n.leading_zeros()
}

pub fn print_flush(message: &str) {
    eprint!("{message}");
    stdout().flush().expect("Couldn't flush stdout.");
}

pub fn get_line() -> String {
    let mut input = String::new();
    stdin()
        .read_line(&mut input)
        .expect("Couldn't read line from stdin.");
    input
}

pub fn pause() {
    print_flush(&format!("[Enter]"));
    get_line();
}

pub fn ask_y_n() -> bool {
    loop {
        print_flush(&format!("[{GREEN}Y{RESET}/{RED}N{RESET}]: "));
        match get_line().trim().to_uppercase().chars().next() {
            Some('Y') => return true,
            Some('N') => return false,
            None => return false,
            _ => {}
        }
    }
}
