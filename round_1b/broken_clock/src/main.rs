use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let t: usize = read();

    for i in 0..t {
        let v = {
            read::<String>()
                .split_ascii_whitespace()
                .map(|x| x.parse::<u128>().unwrap())
                .collect::<Vec<u128>>()
        };

        let nano = 1_000_000_000;
        let perm = [
            [v[0], v[1], v[2]],
            [v[0], v[2], v[1]],
            [v[1], v[0], v[2]],
            [v[1], v[2], v[0]],
            [v[2], v[0], v[1]],
            [v[2], v[1], v[0]],
        ];

        for v in perm.iter() {
            let mut found = false;
            for d in 0..360 {
                let a = if v[0] < 120 * nano * d {
                    v[0] + 120 * 360 * nano
                } else {
                    v[0]
                } - 120 * nano * d;
                let b = if v[1] < 120 * nano * d {
                    v[1] + 120 * 360 * nano
                } else {
                    v[1]
                } - 120 * nano * d;
                let c = if v[2] < 120 * nano * d {
                    v[2] + 120 * 360 * nano
                } else {
                    v[2]
                } - 120 * nano * d;

                let h = a / (3600 * nano);
                let h_remains = a % (3600 * nano);

                if h_remains != b / 12 {
                    continue;
                }

                let m = b / (12 * 60 * nano);
                let m_remains = (b % (12 * 60 * nano)) / 12;

                if m_remains != c / 720 {
                    continue;
                }

                let s = c / (720 * nano);
                let s_remains = (c % (720 * nano)) / 720;

                if s_remains != 0 {
                    continue;
                }

                println!("Case #{}: {} {} {} {}", i + 1, h, m, s, s_remains);
                found = true;
                break;
            }
            if found {
                break;
            }
        }
    }
}
