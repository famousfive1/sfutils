use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn print_help() {
    println!("Usage: sfu [op] [args] <options>");
}

pub fn read_file(filename: &String) -> Vec<String> {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => println!("Read file {}", display),
    };

    return s.lines().map(|line| line.to_string()).collect();
}

pub fn num_digits(mut n: usize) -> usize {
    let mut ans = 0;
    while n > 0 {
        ans += 1;
        n /= 10;
    }
    ans
}
