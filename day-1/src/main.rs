use std::collections::BinaryHeap;

pub struct LocationGroup<'a> {
    left: &'a [u64],
    right: &'a [u64],
}

impl<'a> LocationGroup<'a> {
    pub fn new(left: &'a [u64], right: &'a [u64]) -> Self {
        Self { left, right }
    }
}


fn main() {
    let mut left = BinaryHeap::from([3, 4, 2, 1, 3, 3]);
    let right = BinaryHeap::from([4, 3, 5, 3, 9, 3]);
    let location_group = LocationGroup::new(left.as_slice(), right.as_slice());

    let result = location_group.left.iter()
        .zip(location_group.right)
        .map(|(l, r)| l.abs_diff(*r))
        .reduce(|a, b| a + b)
        .expect("Operation successful");

    println!("Result: {result}");
}
