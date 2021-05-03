use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let t: usize = read();

    for x in 0..t {
        let (n, q): (usize, usize) = {
            let s: String = read();
            let mut sp = s.split_ascii_whitespace();
            (
                sp.next().unwrap().parse().unwrap(),
                sp.next().unwrap().parse().unwrap(),
            )
        };

        let answers: Vec<(Vec<char>, usize)> = (0..n)
            .map(|_| {
                let s: String = read();
                let mut sp = s.split_ascii_whitespace();
                (
                    sp.next().unwrap().chars().collect(),
                    sp.next().unwrap().parse().unwrap(),
                )
            })
            .collect();

        let mut p = 0;
        let mut s = String::new();
        for i in 0..q {
            let mut t = 0;
            let mut f = 0;
            for j in 0..n {
                if answers[j].0[i] == 'T' {
                    t += answers[j].1;
                    f += q - answers[j].1;
                } else {
                    f += answers[j].1;
                    t += q - answers[j].1;
                }
            }
            if t >= f {
                s.push('T');
                p += t;
            } else {
                s.push('F');
                p += f;
            }
        }

        let l = gcd(p, q);

        println!("Case #{}: {} {}/{}", x + 1, s, p / l, q / l)
    }
}
