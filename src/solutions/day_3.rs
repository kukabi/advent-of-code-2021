use itertools::Itertools;

pub fn day_3_a() {
    let gamma_binary_str = include_str!("../../input/day_3.txt")
        .lines()
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
        .map(|column_sum| if column_sum >= 0 { "1" } else { "0" })
        .join("");
    let gamma = u32::from_str_radix(&gamma_binary_str, 2).unwrap();
    let epsilon = !gamma & ((1 << gamma_binary_str.len()) - 1);
    println!("{}", gamma * epsilon);
}
