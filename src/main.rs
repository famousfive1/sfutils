use std::env;

mod diff;
mod explorer;
mod fuzzy;
mod help;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help::print_help();
        return;
    }

    let op = args[1].clone();

    match op.as_str() {
        "d" | "diff" => diff::diff_file(args),
        "f" | "fuzzy" => fuzzy::fuzzy_file(args),
        "e" | "explore" | "explorer" => explorer::explore(args),
        _ => help::print_help(),
    }
}
