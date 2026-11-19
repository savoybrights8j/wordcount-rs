use std::env;
use std::fs;
use std::io::{self, Read};

struct Stats {
    lines: usize,
    words: usize,
    chars: usize,
    longest: usize,
}

fn analyze(text: &str) -> Stats {
    let lines: Vec<&str> = text.lines().collect();
    Stats {
        lines: lines.len(),
        words: text.split_whitespace().count(),
        chars: text.chars().count(),
        longest: lines.iter().map(|line| line.chars().count()).max().unwrap_or(0),
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let text = if let Some(path) = args.first() {
        fs::read_to_string(path).unwrap_or_else(|err| {
            eprintln!("cannot read {}: {}", path, err);
            std::process::exit(1);
        })
    } else {
        let mut buf = String::new();
        io::stdin().read_to_string(&mut buf).expect("stdin read");
        buf
    };
    let stats = analyze(&text);
    println!("lines {} words {} chars {} longest {}",
             stats.lines, stats.words, stats.chars, stats.longest);
}
