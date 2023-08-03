use std::{path::{Path, PathBuf}, fs::{self}, cmp::Ordering};

use console::Term;

#[derive(Debug)]
struct Exp<'a> {
    term: &'a Term,
    path: &'a mut PathBuf,
    entries: &'a mut Vec<(String, bool)>,
    pointer: usize,
    top: usize,
    bot: usize,
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
        top: 1,
        bot: 5,
    };
    term.hide_cursor().unwrap();
    app.update_entries();

    loop {
        let ch = term.read_char().unwrap();
        match ch {
            'q' => break,
            '?' => app.print_explorer_help(),
            'e' | '\n' => app.enter_directory(),
            'k' => app.dec_pointer(),
            'j' => app.inc_pointer(),
            'r' => app.update_entries(),
            _ => {},
        };
    }
}

impl<'a> Exp<'a> {
    fn update_entries(&mut self) {
        *self.entries = Self::get_entries(&self.path);
        *self.entries = Self::get_entries(self.path);
        self.pointer = 1;
        self.top = 1;
        let (ht, _) = self.term.size();
        let ht: usize = (ht - 5).into();
        self.bot = self.top + ht;
        if self.entries.len() < self.bot {
            self.bot = self.entries.len();
        }
        self.render();
    }

    fn enter_directory(&mut self) {
        if !self.entries[self.pointer - 1].1 {
            return;
        }

        *self.path = self.path.join(self.entries[self.pointer - 1].0.clone()).canonicalize().unwrap();
        self.update_entries();
    }

    fn render(&mut self) {
        let _ = self.term.clear_screen();
        Self::print_banner(&self.path);
        self.print_dir();
    }

    fn inc_pointer(&mut self) {
        if self.pointer == self.entries.len() {
            return;
        }
        self.pointer = self.pointer + 1;
        self.render();
    }

    fn dec_pointer(&mut self) {
        if self.pointer == 1 {
            return;
        }
        self.pointer = self.pointer - 1;
        self.render();
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

    fn print_banner(path: &Path) {
        println!("Press ? for help, q to quit");
        println!("Current directory: {}", path.to_str().unwrap());
        println!("-----------------------------");
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

    fn print_dir(&mut self) {
        let diff = self.bot - self.top;
        if self.pointer < self.top {
            self.top = self.pointer;
            self.bot = self.top + diff;
        }
        else if self.pointer > self.bot {
            self.bot = self.pointer;
            self.top = self.bot - diff;
        }

        let mut row = self.top;

        for i in &self.entries[(self.top-1)..(self.bot)] {
            println!(" {} {} {}",
                if row == self.pointer {"->"} else {"  "},
                if i.1 {"F"} else {" "},
                i.0);
            row += 1;
        }
    }

    fn print_explorer_help(&mut self) {
        self.term.clear_screen().unwrap();
        println!("? - This help menu");
        println!("q - Exit");
        println!("e - Enter directory by pointing to folder and press enter");
        println!("r - Refresh content (also resizes to terminal)");
        println!("\nPress any key to return");
        self.term.read_char().unwrap();
        self.render();
    }
}
