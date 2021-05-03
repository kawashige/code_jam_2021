use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let t: usize = read();

    for q in 0..t {
        let n: u64 = read();

        let mut found = false;
        for i in (n + 1).. {
            let s = i.to_string();
            for j in 1..s.len() {
                let mut x = (&s[..j]).parse::<usize>().unwrap();
                let mut s_x = x.to_string();
                while s_x.len() < s.len() {
                    x += 1;
                    s_x += &x.to_string();
                }
                if s_x == s {
                    println!("Case #{}: {}", q + 1, i);
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
}
