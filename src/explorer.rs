use std::{path::{Path, PathBuf}, fs::{self}};

use console::Term;

use crate::help;

pub fn explore(args: Vec<String>) {
    let mut cur_dir = String::from(".");
    if args.len() > 2 {
        cur_dir = args[2].clone();
    }

    let mut path = Path::new(&cur_dir).canonicalize().unwrap();
    match path.try_exists() {
        Ok(true) => {},
        Ok(false) | Err(_) => {
            println!("Cannot find directory: {}", cur_dir);
            return;
        }
    }

    if !path.is_dir() {
        println!("Is not a directory: {}", cur_dir);
        return;
    }

    let term = Term::stdout();
    let mut entries = render(&term, &path);

    loop {
        let ch = term.read_char().unwrap();
        let rerender: bool = match ch {
            'q' => break,
            '?' => { print_explorer_help(&term); true },
            'e' => { enter_directory(&mut path, &term, &entries); true },
            _ => false,
        };

        if rerender {
            entries = render(&term, &path);
        }
    }
}

fn enter_directory(path: &mut PathBuf, term: &Term, entries: &Vec<(String, bool)>) {
    term.write_line("Enter #: ").unwrap();
    let num = term.read_line().unwrap();
    let num: usize = match num.parse() {
        Ok(x) => x,
        Err(_) => {
            term.write_line("Not a number. Press any key to continue.").unwrap();
            term.read_char().unwrap();
            return;
        }
    };

    if num < 1 || num > entries.len() {
        term.write_line("Number out of range. Press any key to continue.").unwrap();
        term.read_char().unwrap();
        return;
    }

    if !entries[num-1].1 {
        term.write_line("Not a directory. Press any key to continue.").unwrap();
        term.read_char().unwrap();
        return;
    }

    *path = path.join(entries[num-1].0.clone()).canonicalize().unwrap();
}

fn render(term: &Term, path: &PathBuf) -> Vec<(String, bool)> {
    let _ = term.clear_screen();
    print_banner(&term, &path);
    let entries = read_dir(&path);
    print_dir(&term, &entries);
    entries
}


fn print_banner(term: &Term, path: &Path) {
    term.write_line("Press ? for help, q to quit").unwrap();
    term.write_line(format!("Current directory: {}", path.to_str().unwrap()).as_str()).unwrap();
    term.write_line("-----------------------------").unwrap();
}

fn read_dir(path: &PathBuf) -> Vec<(String, bool)> {
    let read_dir = fs::read_dir(path).unwrap();
    let mut entries: Vec<(String, bool)> = vec![];
    entries.push(("..".to_string(), true));
    entries.push((".".to_string(), true));
    for i in read_dir {
        let j = i.unwrap();
        entries.push((j.file_name().to_str().unwrap().to_string(), j.file_type().unwrap().is_dir()));
    }
    entries
}


fn print_dir(term: &Term, entries: &Vec<(String, bool)>) {
    let mw = help::num_digits(entries.len() + 2);

    let mut row = 1;
    // term.write_line(format!("{:>mw$} F ../", row, mw=mw).as_str()).unwrap(); row += 1;
    // term.write_line(format!("{:>mw$} F ./", row, mw=mw).as_str()).unwrap(); row += 1;

    for i in entries {
        let _ = term.write_line(
            format!("{:>mw$} {} {}",
                row,
                if i.1 {'F'} else {' '},
                i.0,
                mw=mw
            ).as_str()
        );
        row += 1;
    }

    let _ = term.write_line("");
}

fn print_explorer_help(term: &Term) {
    let _ = term.clear_screen().unwrap();
    let _ = term.write_line("? - This help menu").unwrap();
    let _ = term.write_line("q - Exit").unwrap();
    let _ = term.write_line("e - Enter directory by typing number and pressing enter").unwrap();
    let _ = term.write_line("\nPress any key to return").unwrap();
    let _ = term.read_char().unwrap();
}
