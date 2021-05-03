use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let t: usize = read();

    for i in 0..t {
        let n: usize = read();
        let cards: Vec<Vec<u64>> = (0..n)
            .map(|_| {
                read::<String>()
                    .split_ascii_whitespace()
                    .map(|sp| sp.parse().unwrap())
                    .collect()
            })
            .collect();

        let sum = cards.iter().map(|v| v[0] * v[1]).sum::<u64>();

        let mut result = 0;
        for j in (0..sum).rev() {
            let mut target = j;
            let mut sum_target = sum;

            let mut not_target = false;
            for j in 0..n {
                let mut max = cards[j][1];
                while target % cards[j][0] == 0 {
                    if max == 0 {
                        not_target = true;
                        break;
                    }
                    max -= 1;
                    target /= cards[j][0];
                    sum_target -= cards[j][0];
                }
                if not_target {
                    break;
                }
            }

            if target == 1 && j == sum_target {
                result = j;
                break;
            }
        }

        println!("Case #{}: {}", i + 1, result);
    }
}
