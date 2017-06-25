// This would be much better with inclusive ranges
pub fn sum_of_squares(n: i32) -> i32 {
    let upper = n + 1;
    (0..upper).map(|x| x.pow(2)).sum::<i32>()
}

pub fn square_of_sum(n: i32) -> i32 {
    let upper = n + 1;
    (0..upper).sum::<i32>().pow(2)
}

pub fn difference(n: i32) -> i32 {
    (square_of_sum(n) - sum_of_squares(n)).abs()
}
