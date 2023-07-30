use std::env;

mod diff;
mod help;
mod fuzzy;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        help::print_help();
        return;
    }

    let op = args[2].clone();

    match op.as_str() {
        "d" | "diff" => diff::diff_file(args),
        "f" | "fuzzy" => fuzzy::fuzzy_file(args),
        _ => help::print_help(),
    }
}
