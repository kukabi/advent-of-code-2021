const SIZE: usize = 5;

fn board_has_finished(board: &[[u16; SIZE]; SIZE]) -> bool {
    for i in 0..SIZE {
        if board[i].iter().sum::<u16>() == 0 {
            return true;
        }
        let mut column_sum = 0;
        for row in board {
            column_sum += row[i];
        }
        if column_sum == 0 {
            return true;
        }
    }
    false
}

pub fn day_4_a() {
    let start_time = std::time::Instant::now();
    let parts: Vec<&str> = include_str!("../../input/day_4.txt")
        .split("\n\n")
        .collect();
    let numbers: Vec<u16> = parts[0]
        .split(',')
        .map(|str| str.parse().unwrap())
        .collect();
    let mut boards = vec![];
    for board_str in parts.iter().skip(1) {
        let mut board = [[0u16; SIZE]; SIZE];
        for (row_index, line) in board_str.split('\n').enumerate() {
            line.split_whitespace()
                .map(|str| str.parse().unwrap())
                .enumerate()
                .for_each(|(number_index, number)| board[row_index][number_index] = number);
        }
        boards.push(board);
    }
    for number in numbers {
        for board in &mut boards {
            for row in &mut board.iter_mut() {
                if let Some(index) = row.iter().position(|a| *a == number) {
                    row[index] = 0;
                }
            }
            if board_has_finished(board) {
                let sum: u16 = board.iter().map(|row| row.iter().sum::<u16>()).sum();
                let result = number * sum;
                let elapsed_time = start_time.elapsed();
                println!("Result: {}. Elapsed time: {:?}.", result, elapsed_time);
                return;
            }
        }
    }
    unreachable!();
}
