use crate::help::{self, read_file};

fn num_digits(mut n: usize) -> usize {
    let mut ans = 0;
    while n > 0 {
        ans += 1;
        n /= 10;
    }
    ans
}

fn fuzzy_find(file: &Vec<String>, pattern: &String, case_insens: bool) -> Vec<String> {
    let mut ans: Vec<String> = vec![];
    let mw = num_digits(file.len());

    for (i, line) in file.iter().enumerate() {
        if find_in_line(line, pattern, case_insens) {
            ans.push(format!("{:>mw$} {}", i, line.clone(), mw=mw));
        }
    }

    ans
}

fn find_in_line(line: &str, pattern: &str, case_insens: bool) -> bool {
    let line: Vec<char> = line.chars()
        .map(|c| if case_insens {c.to_ascii_lowercase()} else {c})
        .collect();
    let pattern: Vec<char> = pattern.chars()
        .map(|c| if case_insens {c.to_ascii_lowercase()} else {c})
        .collect();
    let n = line.len();
    let m = pattern.len();
    let mut i = 0;
    let mut j = 0;

    while i < n && j < m {
        if line[i] == pattern[j] {
            j += 1;
        }
        i += 1;
    }

    j == m
}

pub fn fuzzy_file(args : Vec<String>) {
    if args.len() > 5 {
        help::print_help();
        return;
    }

    let mut case_insens = false;
    if args.len() == 5 {
        match args[4].as_str() {
            "i" => case_insens = true,
            _ => {
                help::print_help();
                return;
            }
        }
    }

    let file = read_file(&args[1]);
    let found_lines = fuzzy_find(&file, &args[3], case_insens);

    if case_insens {
        println!("Fuzzy find case-insensitive: {}", &args[3]);
    }
    else {
        println!("Fuzzy find : {}", &args[3]);
    }
    println!("Found {} matches", found_lines.len());
    println!("------");
    for line in &found_lines {
        println!("{line}");
    }
}

