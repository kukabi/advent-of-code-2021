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
    let median = nums[nums.len() / 2];
    let result = nums.iter().map(|x| (x - median).abs()).sum::<i32>();
    println!("7a: {}", result);
}

pub fn day_7_b() {
    fn consumption(x: i32) -> i32 {
        if x == 0 {
            0
        } else {
            x + consumption(x - 1)
        }
    }
    let nums: Vec<i32> = include_str!("../../input/day_7.txt")
        .split(',')
        .map(|num| num.parse().unwrap())
        .collect();
    let avg_floor = (nums.iter().sum::<i32>() as f32 / nums.len() as f32).floor() as i32;
    println!(
        "7b: {}",
        (avg_floor..(avg_floor + 2))
            .map(|a| nums.iter().map(|x| consumption((x - a).abs())).sum::<i32>())
            .min()
            .unwrap()
    );
}
