use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let n: usize = read();
    let mut inputs = (0..n)
        .map(|_| {
            let s: String = read();
            let mut sp = s.split_ascii_whitespace();
            let x = sp.next().unwrap().parse::<usize>().unwrap();
            let y = sp.next().unwrap().parse::<usize>().unwrap();
            let chars = sp.next().unwrap().chars().collect::<Vec<char>>();
            (x, y, chars)
        })
        .collect::<Vec<(usize, usize, Vec<char>)>>();

    for i in 0..n {
        let input = inputs.remove(0);
        let l = input.2.len();
        let mut prev = if input.2[0] == '?' {
            None
        } else {
            Some(input.2[0])
        };
        let mut cost = 0;
        for j in 1..l {
            match (prev, input.2[j]) {
                (Some('C'), 'J') => {
                    cost += input.0;
                }
                (Some('J'), 'C') => {
                    cost += input.1;
                }
                _ => {}
            }
            if input.2[j] != '?' {
                prev = Some(input.2[j]);
            }
        }

        println!("Case #{}: {}", i + 1, cost);
    }
}
