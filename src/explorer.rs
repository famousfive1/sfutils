use std::{path::{Path, PathBuf}, fs::{self}, cmp::{max, min}};

use console::Term;

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
    let mut entries = get_entries(&path);
    let mut pointer = 1;

    loop {
        render(&term, &path, pointer, &entries);
        let ch = term.read_char().unwrap();
        match ch {
            'q' => break,
            '?' => print_explorer_help(&term),
            'e' => enter_directory(&mut path, &mut entries, pointer),
            'j' => pointer = min(pointer+1, entries.len()),
            'k' => pointer = max(pointer-1, 1),
            _ => {},
        };

    }
}

fn get_entries(path: &PathBuf) -> Vec<(String, bool)> {
    let entries = read_dir(&path);
    entries
}

fn enter_directory(path: &mut PathBuf, entries: &mut Vec<(String, bool)>, pointer: usize) {
    if !entries[pointer - 1].1 {
        return;
    }

    *path = path.join(entries[pointer - 1].0.clone()).canonicalize().unwrap();
    *entries = get_entries(path);
}

fn render(term: &Term, path: &PathBuf, pointer: usize, entries: &Vec<(String, bool)>) {
    let _ = term.clear_screen();
    print_banner(&term, &path);
    print_dir(&term, &entries, pointer);
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


fn print_dir(term: &Term, entries: &Vec<(String, bool)>, pointer: usize) {
    let mut row = 1;

    for i in entries {
        let _ = term.write_line(
            format!(" {} {} {}",
                if row == pointer {"->"} else {"  "},
                if i.1 {"F"} else {" "},
                i.0,
            ).as_str()
        );
        row += 1;
    }

    term.write_line("").unwrap();
}

fn print_explorer_help(term: &Term) {
    term.clear_screen().unwrap();
    term.write_line("? - This help menu").unwrap();
    term.write_line("q - Exit").unwrap();
    term.write_line("e - Enter directory by typing number and pressing enter").unwrap();
    term.write_line("\nPress any key to return").unwrap();
    term.read_char().unwrap();
}
