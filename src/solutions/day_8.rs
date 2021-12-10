pub fn all() {
    day_8_a();
    day_8_b();
}

pub fn day_8_a() {
    let search_len = vec![2usize, 3, 4, 7];
    println!(
        "8a: {}",
        include_str!("../../input/day_8.txt")
            .lines()
            .map(|line| {
                line.split_once(" | ")
                    .unwrap()
                    .1
                    .split_whitespace()
                    .filter(|pattern| search_len.contains(&pattern.len()))
                    .count()
            })
            .sum::<usize>()
    );
}

pub fn day_8_b() {}
