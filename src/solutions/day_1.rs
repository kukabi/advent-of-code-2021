use crate::util::parse_input;
use itertools::Itertools;

pub fn all() {
    day_1_a();
    day_1_b();
}

pub fn day_1_a() {
    println!(
        "1a: {}",
        parse_input::<u16>("./input/day_1.txt")
            .tuple_windows()
            .fold(0, |count, (a, b)| { count + (b > a) as u16 })
    );
}

pub fn day_1_b() {
    println!(
        "1b: {}",
        parse_input::<u16>("./input/day_1.txt")
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(a, b)| b > a)
            .count()
    );
}
