use console::style;

use crate::help;

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

fn get_diff_lines(a: &Vec<String>, b: &Vec<String>, full: bool, dp: &Vec<Vec<usize>>) -> (Vec<String>, usize, usize, usize) {
    let n = a.len();
    let m = b.len();
    let mw = 2 * help::num_digits(if n > m { n } else { m });
    let mut ans: Vec<String> = Vec::new();
    let mut i = n;
    let mut j = m;
    let mut add = 0;
    let mut del = 0;

    while i > 0 && j > 0 {
        if a[i - 1] == b[j - 1] {
            if full {
                ans.push(format!("{:<mw$}    {}", i, a[i - 1], mw=mw));
            }
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            ans.push(style(format!("{:<mw$} -- {}", i, a[i - 1], mw=mw)).red().to_string());
            del += 1;
            i -= 1;
        } else {
            ans.push(style(format!("{:>mw$} ++ {}", j, b[j - 1], mw=mw)).green().to_string());
            add += 1;
            j -= 1;
        }
    }

    while i > 0 {
        ans.push(style(format!("{:<mw$} -- {}", i, a[i - 1], mw=mw)).red().to_string());
        i -= 1;
    }

    while j > 0 {
        ans.push(style(format!("{:>mw$} ++ {}", j, b[j - 1], mw=mw)).green().to_string());
        j -= 1;
    }

    ans.reverse();
    (ans, add, del, dp[n][m])
}

pub fn diff_file(args : Vec<String>) {
    if args.len() < 4 {
        help::print_help();
        return;
    }

    let mut full = false;
    if args.len() > 4 {
        match args[4].as_str() {
            "f" | "full" => full = true,
            _ => {
                help::print_help();
                return;
            }
        };
    }

    let s1 = help::read_file(&args[2]);
    let s2 = help::read_file(&args[3]);

    let dp = diff(&s1, &s2);
    let (diff, add, del, same) = get_diff_lines(&s1, &s2, full, &dp);

    println!("# Additions: {}; Deletions: {}; Same: {}", add, del, same);
    println!("Diff\n----");
    for l in diff {
        println!("{}", l);
    }
}
