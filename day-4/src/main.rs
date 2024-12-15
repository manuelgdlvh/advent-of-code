use anyhow::bail;

const ROW_SIZE: usize = 10;
const COLUMN_SIZE: usize = 10;


pub fn main() {
    let input: [[char; COLUMN_SIZE]; ROW_SIZE] = [
        ['.', '.', '.', '.', 'X', 'X', 'M', 'A', 'S', '.'],
        ['.', 'S', 'A', 'M', 'X', 'M', 'S', '.', '.', '.'],
        ['.', '.', '.', 'S', '.', '.', 'A', '.', '.', '.'],
        ['.', '.', 'A', '.', 'A', '.', 'M', 'S', '.', 'X'],
        ['X', 'M', 'A', 'S', 'A', 'M', 'X', '.', 'M', 'M'],
        ['X', '.', '.', '.', '.', '.', 'X', 'A', '.', 'A'],
        ['S', '.', 'S', '.', 'S', '.', 'S', '.', 'S', 'S'],
        ['.', 'A', '.', 'A', '.', 'A', '.', 'A', '.', 'A'],
        ['.', '.', 'M', '.', 'M', '.', 'M', '.', 'M', 'M'],
        ['.', 'X', '.', 'X', '.', 'X', 'M', 'A', 'S', 'X'],
    ];


    let mut result = 0;
    let word = "XMAS";
    let initial_char = 'X';
    let rest_chars = &['M', 'A', 'S'];

    for i in 0..ROW_SIZE {
        for j in 0..COLUMN_SIZE {
            if initial_char != input[i][j] {
                continue;
            }

            let mut movements: [Option<MatrixDirection>; 8] = MatrixDirection::all();
            for movement_num in 1..word.len() {
                movements.iter_mut()
                    .for_each(|val| {
                        if val.is_none() {
                            return;
                        }

                        let mut result = false;
                        if let Ok((new_i, new_j)) = val.unwrap().move_(i, j, movement_num as isize) {
                            result = input[new_i][new_j].eq(&rest_chars[movement_num - 1]);
                        }

                        if !result {
                            *val = None;
                        }
                    })
            }

            movements.iter()
                .filter(|val| val.is_some())
                .for_each(|val| {
                    println!("Starting from point: {i} - {j} in {:?} direction", val.unwrap());
                    result += 1;
                });
        }
    }

    println!("{word} word found {result} times");
}


#[derive(Copy, Clone, Debug)]
pub enum MatrixDirection {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl MatrixDirection {
    pub fn all() -> [Option<MatrixDirection>; 8] {
        [
            Some(MatrixDirection::Up),
            Some(MatrixDirection::Down),
            Some(MatrixDirection::Left),
            Some(MatrixDirection::Right),
            Some(MatrixDirection::UpLeft),
            Some(MatrixDirection::UpRight),
            Some(MatrixDirection::DownLeft),
            Some(MatrixDirection::DownRight),
        ]
    }
    pub fn move_(&self, i: usize, j: usize, movement_num: isize) -> anyhow::Result<(usize, usize)> {
        let mut new_i: isize = i as isize;
        let mut new_j: isize = j as isize;
        match self {
            MatrixDirection::Up => {
                new_i -= movement_num;
            }
            MatrixDirection::Down => {
                new_i += movement_num;
            }
            MatrixDirection::Left => {
                new_j -= movement_num;
            }
            MatrixDirection::Right => {
                new_j += movement_num;
            }
            MatrixDirection::UpRight => {
                new_i -= movement_num;
                new_j += movement_num;
            }
            MatrixDirection::DownRight => {
                new_i += movement_num;
                new_j += movement_num;
            }
            MatrixDirection::UpLeft => {
                new_i -= movement_num;
                new_j -= movement_num;
            }
            MatrixDirection::DownLeft => {
                new_i += movement_num;
                new_j -= movement_num;
            }
        }

        if new_i < 0 || new_i >= ROW_SIZE as isize
            || new_j < 0 || new_j >= COLUMN_SIZE as isize {
            bail!("INVALID_MOVEMENT");
        }

        Ok((new_i as usize, new_j as usize))
    }
}





