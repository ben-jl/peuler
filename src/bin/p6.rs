

pub fn main() -> () {
    let mut sum_squares : i32 = 0;
    let mut sum_to_be_squared : i32 = 0;

    for i in 1..=100 {
        sum_squares += i*i;
        sum_to_be_squared += i;
    }
    let squared_sum = sum_to_be_squared * sum_to_be_squared;
    let result = squared_sum - sum_squares;
    println!("{} - {} = {}", squared_sum, sum_to_be_squared, result);
}