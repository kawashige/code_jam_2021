use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let t: usize = read();

    for x in 0..t {
        let n: usize = read();
        let mut nums = read::<String>()
            .split_ascii_whitespace()
            .map(|s| {
                s.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect::<Vec<u8>>()
            })
            .collect::<Vec<Vec<u8>>>();

        let mut count = 0;
        let mut prev = nums.remove(0);
        let mut prev_l = prev.len();

        for _ in 1..n {
            let mut current = nums.remove(0);
            let current_l = current.len();

            if prev_l == current_l {
                if prev >= current {
                    current.push(0);
                    count += 1;
                }
            } else if prev_l > current_l {
                if &prev[..current_l] > &current {
                    count += prev_l - current_l + 1;
                    (0..(prev_l - current_l + 1)).for_each(|_| current.push(0));
                } else if &prev[..current_l] < &current {
                    count += prev_l - current_l;
                    (0..(prev_l - current_l)).for_each(|_| current.push(0));
                } else {
                    if let Some(index) = (current_l..prev_l).rev().find(|j| prev[*j] != 9) {
                        count += prev_l - current_l;
                        (current_l..index).for_each(|j| current.push(prev[j]));
                        current.push(prev[index] + 1);
                        ((index + 1)..prev_l).for_each(|_| current.push(0));
                    } else {
                        count += prev_l - current_l + 1;
                        (0..(prev_l - current_l + 1)).for_each(|_| current.push(0));
                    }
                }
            }
            prev = current;
            prev_l = prev.len();
        }

        println!("Case #{}: {}", x + 1, count);
    }
}
