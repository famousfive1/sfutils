use std::{path::{Path, PathBuf}, fs::{self}, cmp::{max, min, Ordering}};

use console::Term;

#[derive(Debug)]
struct Exp<'a> {
    term: &'a Term,
    path: &'a mut PathBuf,
    entries: &'a mut Vec<(String, bool)>,
    pointer: usize,
}

pub fn explore(args: Vec<String>) {
    let mut cur_dir = String::from(".");
    if args.len() > 2 {
        cur_dir = args[2].clone();
    }

    let path = Path::new(&cur_dir).canonicalize().unwrap();
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

    start_app(path);
}

fn start_app(path: PathBuf) {
    let term = Term::stdout();
    let mut app = Exp {
        term: &term,
        path: &mut path.clone(),
        entries: &mut vec![],
        pointer: 1,
    };

    loop {
        app.update_entries();
        app.render();
        let ch = term.read_char().unwrap();
        match ch {
            'q' => break,
            '?' => app.print_explorer_help(),
            'e' | '\n' => app.enter_directory(),
            'k' => app.dec_pointer(),
            'j' => app.inc_pointer(),
            _ => {},
        };
    }
}

impl<'a> Exp<'a> {
    fn update_entries(&mut self) {
        *self.entries = Self::get_entries(&self.path);
    }

    fn enter_directory(&mut self) {
        if !self.entries[self.pointer - 1].1 {
            return;
        }

        *self.path = self.path.join(self.entries[self.pointer - 1].0.clone()).canonicalize().unwrap();
        *self.entries = Self::get_entries(self.path);
        self.pointer = 1;
    }

    fn render(&self) {
        let _ = self.term.clear_screen();
        Self::print_banner(&self.term, &self.path);
        Self::print_dir(&self.term, &self.entries, self.pointer);
    }

    fn inc_pointer(&mut self) {
        self.pointer = min(self.pointer + 1, self.entries.len());
    }

    fn dec_pointer(&mut self) {
        self.pointer = max(1, self.pointer - 1);
    }

    fn get_entries(path: &PathBuf) -> Vec<(String, bool)> {
        let mut entries = Self::read_dir(&path);
        entries.sort_by(|a, b| {
            if a.1 ^ b.1 {
                if a.1 {
                    Ordering::Less
                }
                else {
                    Ordering::Greater
                }
            }
            else {
                a.0.cmp(&b.0)
            }
        });
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

    fn print_explorer_help(&self) {
        self.term.clear_screen().unwrap();
        self.term.write_line("? - This help menu").unwrap();
        self.term.write_line("q - Exit").unwrap();
        self.term.write_line("e - Enter directory by typing number and pressing enter").unwrap();
        self.term.write_line("\nPress any key to return").unwrap();
        self.term.read_char().unwrap();
    }
}
