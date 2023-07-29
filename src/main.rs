use std::io::Read;
use std::fs::File;
use std::path::Path;
use std::env;

fn read_file(filename : &String) -> Vec<String> {
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

fn print_help() {
    println!("Usage: diff <file> <option> <arg> <options>");
}

fn diff(a : &Vec<String>, b : &Vec<String>) -> Vec<Vec<usize>> {
    let n = a.len();
    let m = b.len();

    let mut dp: Vec<Vec<usize>> = vec![vec![0; m + 1]; n + 1];

    // DP to find the LCS of a and b
    for i in 1..n + 1 {
        for j in 1..m + 1 {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else if dp[i - 1][j] > dp[i][j - 1] {
                dp[i][j] = dp[i - 1][j];
            } else {
                dp[i][j] = dp[i][j - 1];
            }
        }
    }

    dp
}

fn get_max_width(mut n : usize) -> usize {
    let mut ans = 0;
    while n > 0 {
        ans += 1;
        n /= 10;
    }
    2 * ans
}

fn get_diff_lines(a: &Vec<String>, b: &Vec<String>, full: bool, dp: &Vec<Vec<usize>>) -> (Vec<String>, usize, usize, usize) {
    let n = a.len();
    let m = b.len();
    let mw = get_max_width(if n > m { n } else { m });
    let mut ans: Vec<String> = Vec::new();
    let mut i = n;
    let mut j = m;
    let mut add = 0;
    let mut del = 0;

    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            if full {
                ans.push(format!("{:mw$}    {}", i, a[i - 1], mw=mw));
            }
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            ans.push(format!("{:<mw$} -- {}", i, a[i - 1], mw=mw));
            del += 1;
            i -= 1;
        } else {
            ans.push(format!("{:>mw$} ++ {}", j, b[j - 1], mw=mw));
            add += 1;
            j -= 1;
        }
    }

    while i > 0 {
        ans.push(format!("{:<mw$} -- {}", i, a[i - 1], mw=mw));
        i -= 1;
    }

    while j > 0 {
        ans.push(format!("{:>mw$} ++ {}", j, b[j - 1], mw=mw));
        j -= 1;
    }

    ans.reverse();
    (ans, add, del, dp[n][m])
}

fn diff_file(args : Vec<String>) {
    let mut full = false;
    if args.len() > 4 {
        if args[4] == "f" || args[4] == "full" {
            full = true;
        }
        else {
            print_help();
            return;
        }
    }

    let s1 = read_file(&args[1]);
    let s2 = read_file(&args[3]);

    let dp = diff(&s1, &s2);
    let (diff, add, del, same) = get_diff_lines(&s1, &s2, full, &dp);

    println!("# Add: {}; Del: {}; Same: {}", add, del, same);
    println!("Diff\n----");
    for l in diff {
        println!("{}", l);
    }
}

fn fuzzy_file(_args : Vec<String>) {
    println!("Fuzzy not implemented");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        print_help();
        return;
    }

    let op = args[2].clone();

    match op.as_str() {
        "d" | "diff" => diff_file(args),
        "f" | "fuzzy" => fuzzy_file(args),
        _ => print_help(),
    }
}
