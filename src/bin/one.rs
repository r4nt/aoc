use std::collections::HashSet;
use std::io::BufRead;

fn main() {
    let v: Vec<i32> = std::io::stdin()
        .lock()
        .lines()
        .filter_map(|l| l.unwrap().parse().ok())
        .collect();
    println!("{}", multiply_sum(v, 2020));
}

fn multiply_sum(entries: Vec<i32>, sum: i32) -> i32 {
    for b in &entries {
        let mut map = HashSet::new();
        for e in &entries {
            if map.contains(&(sum - b - e)) {
                return b * e * (sum - b - e);
            }
            map.insert(e);
        }
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum() {
        let r = multiply_sum(vec![2000, 20], 2020);
        assert_eq!(r, 2000 * 20);
    }
}
