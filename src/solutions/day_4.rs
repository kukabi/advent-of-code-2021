const SIZE: usize = 5;
type Board = [[u16; SIZE]; SIZE];

pub fn day_4_a() {
    let start_time = std::time::Instant::now();
    let (numbers, mut boards) = read_numbers_and_boards();
    for number in numbers {
        for board in &mut boards {
            for row in &mut board.iter_mut() {
                if let Some(index) = row.iter().position(|a| *a == number) {
                    row[index] = 0;
                }
            }
            if let Some(sum) = board_finish_sum(board) {
                let score = number * sum;
                let elapsed_time = start_time.elapsed();
                println!("Score: {}. Elapsed time: {:?}.", score, elapsed_time);
                return;
            }
        }
    }
    unreachable!();
}

pub fn day_4_b() {
    let start_time = std::time::Instant::now();
    let (numbers, mut boards) = read_numbers_and_boards();
    let boards_count = boards.len();
    let mut finished_boards = vec![];
    for number in numbers {
        for (index, board) in boards.iter_mut().enumerate() {
            if finished_boards.contains(&index) {
                continue;
            }
            for row in &mut board.iter_mut() {
                if let Some(index) = row.iter().position(|a| *a == number) {
                    row[index] = 0;
                }
            }
            if let Some(sum) = board_finish_sum(board) {
                finished_boards.push(index);
                if finished_boards.len() == boards_count {
                    let score = number * sum;
                    let elapsed_time = start_time.elapsed();
                    println!("Score: {}. Elapsed time: {:?}.", score, elapsed_time);
                    return;
                }
            }
        }
    }
    unreachable!();
}

fn board_finish_sum(board: &Board) -> Option<u16> {
    fn board_sum(board: &Board) -> u16 {
        board.iter().map(|row| row.iter().sum::<u16>()).sum()
    }
    for i in 0..SIZE {
        if board[i].iter().sum::<u16>() == 0 {
            return Some(board_sum(board));
        }
        let mut column_sum = 0;
        for row in board {
            column_sum += row[i];
        }
        if column_sum == 0 {
            return Some(board_sum(board));
        }
    }
    None
}

fn read_numbers_and_boards() -> (Vec<u16>, Vec<Board>) {
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
    (numbers, boards)
}
