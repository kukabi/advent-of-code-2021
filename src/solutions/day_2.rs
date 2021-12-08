use itertools::Itertools;

pub fn all() {
    day_2_a();
    day_2_b();
}

pub fn day_2_a() {
    let (pos, depth) = include_str!("../../input/day_2.txt")
        .lines()
        .map(|line| {
            let (cmd, units) = line.split_whitespace().collect_tuple().unwrap();
            (cmd, units.parse::<i32>().unwrap())
        })
        .fold((0, 0), |(pos, depth), (cmd, units)| match cmd {
            "forward" => (pos + units, depth),
            "down" => (pos, depth + units),
            "up" => (pos, depth - units),
            _ => unreachable!("Unexpected command: {}", cmd),
        });
    println!("2a: {}", pos * depth);
}

pub fn day_2_b() {
    let (pos, depth, _aim) = include_str!("../../input/day_2.txt")
        .lines()
        .map(|line| {
            let (cmd, units) = line.split_whitespace().collect_tuple().unwrap();
            (cmd, units.parse::<i32>().unwrap())
        })
        .fold((0, 0, 0), |(pos, depth, aim), (cmd, units)| match cmd {
            "forward" => (pos + units, depth + units * aim, aim),
            "down" => (pos, depth, aim + units),
            "up" => (pos, depth, aim - units),
            _ => unreachable!("Unexpected command: {}", cmd),
        });
    println!("2b: {}", pos * depth);
}
