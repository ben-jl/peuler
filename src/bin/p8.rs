use std::collections::*;
use std::iter::*;
fn main() -> std::io::Result<()> {
    let p = std::path::Path::new("./inputs/p8.txt");
    let rawinput = std::fs::read_to_string(p).unwrap();

    let mut digits : Vec<u64> = vec![];
    for c in rawinput.chars().filter(|c| !c.is_whitespace()) {
        let n : u64 = c.to_digit(10).unwrap().into();
        digits.push(n);
    }   

    let mut window : VecDeque<u64> = VecDeque::new();
    let mut current_max_sum : u64 = 0;
    for d in digits {
        if window.len() == 13 {
            let _last_dig = window.pop_back().unwrap();

        }
        window.push_front(d);
        let window_prod = window.iter().fold(1, |acc,nxt| nxt * acc );
        if window_prod > current_max_sum {
            eprintln!("{:?} = {} > {}", &window, &window_prod, &current_max_sum);
            current_max_sum = window_prod;
            
        }
    }
    Ok(())
}