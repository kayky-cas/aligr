use std::{
    env::args,
    io::{stdin, stdout, Write},
    process::exit,
};

fn main() {
    let mut args = args();

    let program_name = args.next().unwrap();

    let align_word = args.next().unwrap_or_else(|| {
        eprintln!("Please provide an alignment word!");
        eprintln!("Usage: {} <word>", program_name);
        exit(1);
    });

    let mut max_align = 0;
    let mut lines_split = Vec::new();

    for pair in stdin().lines().flatten().map(|l| {
        let l = String::leak(l);
        l.split_once(&align_word).unwrap_or_else(|| (l, ""))
    }) {
        if pair.0.len() > max_align {
            max_align = pair.0.len();
        }

        lines_split.push(pair);
    }

    let mut stdout = stdout().lock();

    for (left, right) in lines_split {
        writeln!(
            stdout,
            "{:<width$}{}{}",
            left,
            align_word,
            right,
            width = max_align
        )
        .unwrap_or_else(|err| {
            eprintln!("Error writing to stdout: {}", err);
            exit(1);
        })
    }
}
