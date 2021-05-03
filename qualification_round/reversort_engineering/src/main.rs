use std::str::FromStr;

fn read<T: FromStr>() -> T {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse().ok().unwrap()
}

fn main() {
    let t: usize = read();
    let inputs = (0..t)
        .map(|_| {
            let s: String = read();
            let mut sp = s.split_ascii_whitespace();
            let n = sp.next().unwrap().parse::<usize>().unwrap();
            let c = sp.next().unwrap().parse::<usize>().unwrap();
            (n, c)
        })
        .collect::<Vec<(usize, usize)>>();

    for i in 0..t {
        let (n, c) = (inputs[i].0, inputs[i].1);
        if c < n - 1 || (n + 1) * n / 2 - 1 < c {
            println!("Case #{}: IMPOSSIBLE", i + 1);
            continue;
        }

        let mut nums = (0..n).collect::<Vec<usize>>();
        let mut result = vec![0; n];
        let mut remains = c + 1 - n;

        for j in 0..(n - 1) {
            let k = if remains >= nums.len() {
                nums.len() - 1
            } else {
                remains
            };
            remains -= k;
            nums = nums[..=k]
                .iter()
                .rev()
                .chain(nums[(k + 1)..].iter())
                .cloned()
                .collect();
            result[nums.remove(0)] = j + 1;
        }
        result[nums.remove(0)] = n;

        println!(
            "Case #{}: {}",
            i + 1,
            result
                .into_iter()
                .map(|i| i.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }
}
