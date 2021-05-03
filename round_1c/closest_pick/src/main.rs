use std::collections::BTreeSet;
use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let t: usize = read();

    for i in 0..t {
        let (_n, k) = {
            let v = read::<String>()
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            (v[0], v[1])
        };

        let v = {
            let v = read::<String>()
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<BTreeSet<usize>>();

            v.into_iter().collect::<Vec<usize>>()
        };
        let n = v.len();

        let (mut first, mut second) = (v[0] - 1, k - v[n - 1]);
        if first < second {
            std::mem::swap(&mut first, &mut second);
        }

        for j in 1..v.len() {
            if v[j] == v[j - 1] + 1 {
                continue;
            }
            let num = 1 + (v[j] - v[j - 1] - 2) / 2;

            if num > first {
                second = first;
                first = num;
            } else if num > second {
                second = num;
            }
        }

        let mut max = first + second;

        for j in 1..v.len() {
            if v[j] == v[j - 1] + 1 {
                continue;
            }
            max = std::cmp::max(max, v[j] - v[j - 1] - 1);
        }

        println!("Case #{}: {}", i + 1, max as f64 / k as f64);
    }
}
