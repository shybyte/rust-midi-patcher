use std::iter;
use std::ops::Add;


pub fn repeated<T: Clone>(pattern: &[T], times: usize) -> Vec<T> {
    concat(iter::repeat(pattern.to_vec()).take(times).collect())
}

pub fn add<T>(mut xs: Vec<T>, y: T) -> Vec<T>
    where T: Copy + Add<T, Output=T> {
    for x in &mut xs {
        *x = *x + y;
    }
    xs
}

pub fn concat<T: Clone>(input: Vec<Vec<T>>) -> Vec<T> {
    input.into_iter().flat_map(|x| x).collect()
}

