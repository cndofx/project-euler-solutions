fn main() {
    let result = sum_square_difference(100);
    println!("result: {}", result);
}

fn sum_of_squares(up_to: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in 1..=up_to {
        sum += i.pow(2);
    }
    sum
}

fn square_of_sum(up_to: i64) -> i64 {
    let mut sum: i64 = 0;
    for i in 1..=up_to {
        sum += i;
    }
    sum.pow(2)
}

fn sum_square_difference(up_to: i64) -> i64 {
    square_of_sum(up_to) - sum_of_squares(up_to)
}