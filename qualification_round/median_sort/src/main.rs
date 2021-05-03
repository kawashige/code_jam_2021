use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let (t, n, _q) = {
        let s: String = read();
        let v = s
            .split_ascii_whitespace()
            .map(|w| w.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        (v[0], v[1], v[2])
    };

    for _ in 0..t {
        let mut result = Vec::new();
        while result.len() < n {
            if result.is_empty() {
                let mut ask = vec![1, 2, 3];
                println!("1 2 3");
                let m = read::<i32>();
                if m == -1 {
                    return;
                }
                ask.swap(m as usize - 1, 1);
                result = ask;
            } else {
                let mut s = 0;
                let mut e = result.len() - 1;
                let mut prev_s = s;
                let mut prev_e = e;

                let target = result.len() + 1;

                loop {
                    let ask = vec![result[s], result[e], target];
                    println!("{} {} {}", ask[0], ask[1], ask[2]);
                    let m = read::<i32>();
                    if m == -1 {
                        return;
                    }

                    if m as usize == ask[0] {
                        if s == 0 || prev_s + 1 == s {
                            result.insert(s, target);
                            break;
                        }
                        let count = s - prev_s + 1;
                        prev_e = s;
                        if count == 3 {
                            s = prev_s;
                            e = s + 1;
                        } else {
                            e = s - count / 3;
                            s = prev_s + count / 3;
                        }
                    } else if m as usize == ask[1] {
                        let count = prev_e - e + 1;
                        if e == prev_e && e == result.len() - 1 {
                            result.push(target);
                            break;
                        }
                        if count == 2 {
                            result.insert(prev_e, target);
                            break;
                        }
                        prev_s = e;
                        if count == 3 {
                            s = e;
                            e = e + 1;
                        } else {
                            s = e + count / 3;
                            e = prev_e - count / 3
                        }
                    } else if m as usize == ask[2] {
                        prev_s = s;
                        prev_e = e;
                        if s + 1 == e {
                            result.insert(e, target);
                            break;
                        }
                        let count = e - s + 1;
                        if count == 3 {
                            e = s + 1;
                        } else {
                            s = s + count / 3;
                            e = e - count / 3;
                        }
                    }
                }
            }
        }

        println!(
            "{}",
            result
                .into_iter()
                .map(|r| r.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
        let m = read::<i32>();
        if m == -1 {
            return;
        }
    }
}
