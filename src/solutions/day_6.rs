pub fn all() {
    day_6_a();
}

pub fn day_6_a() {
    let mut count = 0;
    let mut herd: Vec<(i16, i16)> = include_str!("../../input/day_6.txt")
        .split(',')
        .map(|v| (v.parse().unwrap(), 80))
        .collect();
    let mut offspring: Vec<(i16, i16)> = Vec::new();
    while !herd.is_empty() {
        count += herd.len();
        for fish in &herd {
            let mut days = fish.1;
            days -= fish.0 + 1;
            while days > -1 {
                offspring.push((8, days));
                days -= 7;
            }
        }
        herd.clear();
        herd.append(&mut offspring);
    }
    println!("{}", count);
}
