use std::collections::HashSet;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn dfs(
    s: &Vec<char>,
    e: &mut Vec<char>,
    count: usize,
    min_count: &mut usize,
    seen: &mut HashSet<String>,
) {
    let key = e.iter().collect::<String>();
    if seen.contains(&key) {
        return;
    }
    if s == e {
        *min_count = std::cmp::min(*min_count, count);
        return;
    }
    if s.len() + 4 < e.len() {
        return;
    }
    seen.insert(key);

    if e.last() == Some(&'0') {
        e.pop();
        dfs(s, e, count + 1, min_count, seen);
        e.push('0');
    }

    for j in 1..s.len() {
        let mut new_e = std::iter::repeat('1')
            .take(j)
            .chain(e.iter().map(|c| if c == &'1' { '0' } else { '1' }))
            .collect::<Vec<char>>();
        dfs(s, &mut new_e, count + 1, min_count, seen);
    }
}

fn main() {
    let t: usize = read();

    for i in 0..t {
        let (s, mut e) = {
            let mut v = read::<String>()
                .split_ascii_whitespace()
                .map(|x| x.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>();

            (v.remove(0), v.remove(0))
        };

        if s == e {
            println!("Case #{}: 0", i + 1);
            continue;
        } else if s.len() == 1 && s[0] == '0' {
            if e.len() == 1 && e[0] == '1' {
                println!("Case #{}: 1", i + 1);
            } else {
                println!("Case #{}: IMPOSSIBLE", i + 1);
            }
            continue;
        }

        let mut count = std::usize::MAX;

        dfs(&s, &mut e, 0, &mut count, &mut HashSet::new());
        if count == std::usize::MAX {
            println!("Case #{}: IMPOSSIBLE", i + 1);
        } else {
            println!("Case #{}: {}", i + 1, count);
        }
    }
}
