fn main() {
    let result = even_fibs_below_n(4000000);
    println!("{}", result);
}

fn even_fibs_below_n(n : i32) -> i32 {
    let mut sum : i32 = 0;
    let mut last : i32 = 1;
    let mut curr : i32 = 1;

    while curr < n {
        if curr % 2 == 0 {
            sum = sum + curr;
        }
        curr = last + curr;
        last = curr - last;
    }

    sum
}