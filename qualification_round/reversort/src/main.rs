use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let t: usize = read();
    let mut inputs: Vec<Vec<usize>> = (0..t)
        .map(|_| {
            let _: usize = read();
            read::<String>()
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    for k in 0..t {
        let mut input = inputs.remove(0);
        let l = input.len();
        let mut cost = 0;
        for i in 0..(l - 1) {
            let j = (i..l).min_by_key(|j| input[*j]).unwrap();
            cost += j - i + 1;
            input = input[..i]
                .iter()
                .chain(input[i..=j].iter().rev())
                .chain(input[(j + 1)..].iter())
                .cloned()
                .collect();
        }

        println!("Case #{}: {}", k + 1, cost);
    }
}
