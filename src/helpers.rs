use std::{env, fs::File, iter::repeat, path::PathBuf, str::FromStr};

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
    line.trim().split('\t')
}

pub fn next_token<'a, 'b, T: FromStr, E>(
    token_iter: &'a mut impl Iterator<Item = &'b str>,
    bad_token: impl Fn(String) -> E,
) -> Result<Option<T>, E> {
    token_iter
        .next()
        .filter(|token| !token.is_empty())
        .map_or(Ok(None), |token| {
            token
                .parse()
                .map(Some)
                .map_err(|_| bad_token(token.to_owned()))
        })
}

pub fn extend_vec_to<T: Clone>(vec: &mut Vec<T>, item: T, len: usize) {
    if len > vec.len() {
        vec.extend(repeat(item).take(len - vec.len()));
    }
}

pub fn break_string(string: &str, max_line_length: usize) -> String {
    let mut cycle = (0..max_line_length).cycle().skip(1);
    string
        .split_inclusive(|_| cycle.next().unwrap() == 0)
        .collect::<Vec<_>>()
        .join("\n")
}
