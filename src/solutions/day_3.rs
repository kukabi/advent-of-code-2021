use itertools::Itertools;

pub fn day_3_a() {
    let gamma_binary_str = include_str!("../../input/day_3.txt")
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(2).unwrap() as i32)
                .collect()
        })
        .fold(vec![0i32, 0], |mut column_sums, binary_column: Vec<i32>| {
            for (index, val) in binary_column.into_iter().enumerate() {
                if column_sums.get_mut(index).is_some() {
                    let e_val = *column_sums.get_mut(index).unwrap();
                    let _ = std::mem::replace(
                        &mut column_sums[index],
                        e_val + if val == 0 { -1 } else { val },
                    );
                } else {
                    column_sums.push(if val == 0 { -1 } else { val });
                }
            }
            column_sums
        })
        .into_iter()
        .map(|column_sum| if column_sum > 0 { "1" } else { "0" })
        .join("");
    let epsilon_binary_str = gamma_binary_str
        .chars()
        .map(|c| if c == '1' { "0" } else { "1" })
        .join("");
    let gamma = u32::from_str_radix(&gamma_binary_str, 2).unwrap();
    let epsilon = u32::from_str_radix(&epsilon_binary_str, 2).unwrap();
    println!("{}", gamma * epsilon);
}
