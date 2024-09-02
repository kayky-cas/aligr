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

    stdin()
        .lines()
        // this expression returning a `std::io::Lines`
        // may produce an infinite number of `Err` in case of a read error
        .map_while(Result::ok)
        .map(|l| {
            // Leak here because we only live once
            let l = String::leak(l);
            l.split_once(&align_word).unwrap_or((l, ""))
        })
        .for_each(|pair| {
            max_align = pair.0.len().max(max_align);
            lines_split.push(pair);
        });

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
