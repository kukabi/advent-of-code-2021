use std::collections::{HashMap, HashSet};
use itertools::Itertools;

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

pub fn day_8_b() {
    let mut digit_map: HashMap<&str, u32> = HashMap::new();
    digit_map.insert("abcefg", 0);
    digit_map.insert("cf", 1);
    digit_map.insert("acdeg", 2);
    digit_map.insert("acdfg", 3);
    digit_map.insert("bcdf", 4);
    digit_map.insert("abdfg", 5);
    digit_map.insert("abdefg", 6);
    digit_map.insert("acf", 7);
    digit_map.insert("abcdefg", 8);
    digit_map.insert("abcdfg", 9);

    let sum = include_str!("../../input/day_8.txt")
        .lines()
        .map(|line| {
            let mut map = HashMap::<char, char>::new();
            let (displays, outputs) = line.split_once(" | ").unwrap();
            let mut displays_by_length: HashMap<usize, Vec<HashSet<char>>> = HashMap::new();
            for display in displays.split_whitespace() {
                displays_by_length
                    .entry(display.len())
                    .or_insert(Vec::new())
                    .push(display.chars().collect());
            }
            let l_2 = &displays_by_length.get(&2).unwrap()[0];
            let l_3 = &displays_by_length.get(&3).unwrap()[0];
            let l_4 = &displays_by_length.get(&4).unwrap()[0];
            let l_7 = &displays_by_length.get(&7).unwrap()[0];
            // find a
            map.insert(*(l_3 - l_2).iter().next().unwrap(), 'a');
            // find g
            let mut rolling: HashSet<char> = l_4.union(l_3).cloned().collect();
            for l_5 in displays_by_length.get(&5).unwrap() {
                let diff = l_5 - &rolling;
                if diff.len() == 1 {
                    let g = *diff.iter().next().unwrap();
                    map.insert(g, 'g');
                    rolling.insert(g);
                    break;
                }
            }
            // find e
            let e = *(l_7 - &rolling).iter().next().unwrap();
            map.insert(e, 'e');
            // find d
            let mut rolling = HashSet::new();
            for c in map.keys() {
                rolling.insert(*c);
            }
            for l_5 in displays_by_length.get(&5).unwrap() {
                let diff = &(l_5 - &rolling) - l_2;
                if diff.len() == 1 {
                    let d = *diff.iter().next().unwrap();
                    map.insert(d, 'd');
                    rolling.insert(d);
                    break;
                }
            }
            // find b
            let b = *(&(l_7 - &rolling) - l_2).iter().next().unwrap();
            map.insert(b, 'b');
            rolling.insert(b);

            // find f
            for l_6 in displays_by_length.get(&6).unwrap() {
                let diff = l_6 - &rolling;
                if diff.len() == 1 {
                    let f = *diff.iter().next().unwrap();
                    map.insert(f, 'f');
                    rolling.insert(f);
                    break;
                }
            }
            // find c
            let c = *(l_7 - &rolling).iter().next().unwrap();
            map.insert(c, 'c');

            let mut mul = 1;
            let mut sum = 0;
            for digit_str in outputs.split_whitespace().rev() {
                let set = String::from_iter(digit_str.chars().map(
                    |c| *map.get(&c).unwrap()
                ).sorted());
                let digit = *digit_map.get(set.as_str()).unwrap();
                sum += digit * mul;
                mul *= 10;
            }
            sum
        })
        .sum::<u32>();
    println!("8b: {}", sum);
}
