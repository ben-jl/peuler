fn main() {
    let result = calc_multiples_of_3_and_5_below_n(1000);
    println!("{}", result);
}

fn calc_multiples_of_3_and_5_below_n(n : i32) -> i32 {
    let mut sum : i32 = 0;
    for number in (1..(n-1)).rev() {
        if number % 3 == 0 || number % 5 == 0 {
            sum = sum + number;
        }
    }
    return sum;
}