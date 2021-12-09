use itertools::Itertools;

pub fn all() {
    day_7_a();
    day_7_b();
}

pub fn day_7_a() {
    let nums: Vec<i32> = include_str!("../../input/day_7.txt")
        .split(',')
        .map(|num| num.parse().unwrap())
        .sorted()
        .collect();
    let position = nums[nums.len() / 2];
    println!(
        "7a: {}",
        nums.iter().map(|a| (a - position).abs()).sum::<i32>()
    );
}

pub fn day_7_b() {
    todo!("7-a is not ready.")
}
