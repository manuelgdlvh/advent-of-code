#[derive(PartialEq, Eq)]
pub enum RowResult {
    Safe,
    Unsafe,
}

fn main() {
    let input: [[u64; 5]; 6] = [
        [7, 6, 3, 4, 2],
        [1, 2, 7, 8, 9],
        [9, 7, 6, 2, 1],
        [1, 3, 2, 4, 5],
        [8, 6, 4, 4, 1],
        [1, 3, 6, 7, 9],
    ];

    let result = input.iter()
        .map(process_row)
        .filter(|r| RowResult::Safe.eq(r))
        .count();

    println!("{result}");
}

fn process_row(row: &[u64; 5]) -> RowResult {
    let mut current_diff = 0;
    for v in row.chunks(2) {
        match (v.get(0), v.get(1)) {
            (Some(a), Some(b)) => {
                let diff = a.abs_diff(*b);
                if diff > (current_diff + 1) {
                    return RowResult::Unsafe;
                }

                current_diff = diff;
            }
            _ => {}
        };
    }

    RowResult::Safe
}
