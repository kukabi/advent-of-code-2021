use crate::util::parse_input;
use itertools::Itertools;

pub fn day_1_a() {
    println!(
        "{}",
        parse_input::<u16>("./input/day_1.txt")
            .tuple_windows()
            .fold(0, |count, (first, second)| {
                count + if second > first { 1 } else { 0 }
            })
    );
}

pub fn day_1_b() {
    println!(
        "{}",
        parse_input::<u16>("./input/day_1.txt")
            .tuple_windows()
            .map(|a: (u16, u16, u16)| a.0 + a.1 + a.2)
            .tuple_windows()
            .filter(|(a, b)| b > a)
            .count()
    );
}
