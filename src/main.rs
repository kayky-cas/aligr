use std::{
    io::{stdin, Write},
    process::exit,
};

fn main() {
    let align_word = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("Please provide an alignment word!");
        exit(1);
    });

    let lines: Vec<String> = stdin().lines().flatten().collect();

    let max_align = lines
        .iter()
        .filter_map(|l| l.split_once(&align_word))
        .map(|(l, _)| l.len())
        .max()
        .unwrap_or(0);

    let mut stdout = std::io::stdout().lock();

    for line in lines {
        match line.split_once(&align_word) {
            Some((left, right)) => writeln!(
                stdout,
                "{:<width$}{}{}",
                left,
                align_word,
                right,
                width = max_align
            ),
            None => writeln!(stdout, "{}", line),
        }
        .unwrap_or_else(|err| {
            eprintln!("Error writing to stdout: {}", err);
            exit(1);
        })
    }
}
