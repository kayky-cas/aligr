use std::io::stdin;

fn main() {
    let align_word = std::env::args()
        .nth(1)
        .expect("Please provide an alignment word");

    let lines: Vec<String> = stdin().lines().flatten().collect();

    let max_align = lines
        .iter()
        .map(|l| l.split_once(&align_word))
        .flatten()
        .map(|(l, _)| l.len())
        .max()
        .unwrap_or(0);

    for line in lines {
        if let Some((left, right)) = line.split_once(&align_word) {
            println!("{:<width$}{}{}", left, align_word, right, width = max_align);
        } else {
            println!("{}", line)
        }
    }
}
