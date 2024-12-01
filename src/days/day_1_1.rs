pub fn solve(input: String) -> String {
    let (mut left, mut right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|l| l.split_once("   ").unwrap())
        .map(|(l, r)| (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap()))
        .collect();

    left.sort_unstable();
    right.sort_unstable();

    let total: i32 = left
        .into_iter()
        .zip(right)
        .map(|(l, r)| l.max(r) - l.min(r))
        .sum();

    total.to_string()
}
