use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let t: usize = read();

    for i in 0..t {
        let (n, a, b) = {
            let v = read::<String>()
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (v[0], v[1], v[2])
        };

        let u = read::<String>()
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        if u.iter()
            .enumerate()
            .filter(|(_, x)| x > &&0)
            .map(|(i, _)| i)
            .collect::<Vec<usize>>()
            .windows(2)
            .any(|v| {
                !((v[1] - v[0]) % a == 0 || (v[1] - v[0]) % b == 0 || (v[1] - v[0]) % (a + b) == 0)
            })
        {
            println!("Case #{}: IMPOSSIBLE", i + 1);
            continue;
        }

        let mut x = n;
        loop {
            let mut v = vec![0; x];
            if x >= a {
                v[x - a] = 1;
            }
            if x >= b {
                v[x - b] = 1;
            }

            let mut j = x - 1;
            let mut valid = true;
            loop {
                if n <= j {
                    while v[j] > 0 {
                        v[j] -= 1;
                        if j >= a {
                            v[j - a] += 1;
                        }
                        if j >= b {
                            v[j - b] += 1;
                        }
                    }
                } else if u[j] < v[j] {
                    while u[j] < v[j] {
                        v[j] -= 1;
                        if j >= a {
                            v[j - a] += 1;
                        }
                        if j >= b {
                            v[j - b] += 1;
                        }
                    }
                } else if u[j] > v[j] {
                    valid = false;
                    break;
                }
                if j == 0 {
                    break;
                }
                j -= 1;
            }

            if valid {
                println!("Case #{}: {}", i + 1, x + 1);
                break;
            }
            x += 1;
        }
    }
}
