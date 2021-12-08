use itertools::Itertools;

pub fn all() {
    day_3_a();
    day_3_b();
}

pub fn day_3_a() {
    let lines: Vec<&str> = include_str!("../../input/day_3.txt").lines().collect();
    let gamma = u32::from_str_radix(&get_binary_guide_str(&lines, true), 2).unwrap();
    let epsilon = u32::from_str_radix(&get_binary_guide_str(&lines, false), 2).unwrap();
    println!("3a: {}", gamma * epsilon);
}

pub fn day_3_b() {
    let lines: Vec<&str> = include_str!("../../input/day_3.txt").lines().collect();
    let o2_rating = find_rating(&lines, true, 0);
    let co2_rating = find_rating(&lines, false, 0);
    println!("3b: {}", o2_rating * co2_rating);
}

fn get_binary_guide_str(lines: &[&str], pick_ones: bool) -> String {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(2).unwrap() as i32)
                .collect()
        })
        .fold(vec![0i32, 0], |mut column_sums, binary_row: Vec<i32>| {
            for (index, bit) in binary_row.into_iter().enumerate() {
                if column_sums.get_mut(index).is_some() {
                    let sum = *column_sums.get_mut(index).unwrap();
                    let _ = std::mem::replace(
                        &mut column_sums[index],
                        sum + if bit == 0 { -1 } else { bit },
                    );
                } else {
                    column_sums.push(if bit == 0 { -1 } else { bit });
                }
            }
            column_sums
        })
        .into_iter()
        .map(|column_sum| {
            if (column_sum >= 0 && pick_ones) || (column_sum < 0 && !pick_ones) {
                "1"
            } else {
                "0"
            }
        })
        .join("")
}

fn find_rating(lines: &[&str], is_o2: bool, bit_position: usize) -> u32 {
    let binary_guide_str = get_binary_guide_str(lines, is_o2);
    let filtered_lines: Vec<&str> = lines
        .iter()
        .filter(|line| line.chars().nth(bit_position) == binary_guide_str.chars().nth(bit_position))
        .copied()
        .collect();
    if filtered_lines.len() == 1 {
        u32::from_str_radix(&filtered_lines[0].to_string(), 2).unwrap()
    } else {
        find_rating(&filtered_lines, is_o2, bit_position + 1)
    }
}
