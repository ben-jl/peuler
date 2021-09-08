use std::convert::TryInto;


pub fn primes_less_than_n_2(n:u32) -> Vec<u32> {
    let mut ps : Vec::<u32> = Vec::with_capacity(n as usize);
    unimplemented!();
}

pub fn primes_less_than_n(n:u64) -> Vec<u64> {
    if n == 0 || n == 1 {
        return vec![];
    }
    let vsize :usize = (n-1).try_into().unwrap();

    let mut vec : Vec<bool> = Vec::with_capacity(vsize);
    vec.resize(vsize, true);

    let sqrtn : u64 = (n as f64).sqrt() as u64 + 1;

    for i in (2..sqrtn).rev() {
        let idx = i - 2;
        if vec[idx as usize] {
            let mut j = i.pow(2);
            while j <= n {
                let jdx = j - 2;
                vec[jdx as usize] = false;
                j = j + i;
            }
        }
    }

    let mut res : Vec<u64> = Vec::new();
    for (i, x) in vec.iter().enumerate() {
        if *x  {
            res.push(i as u64 + 2)
        }
    }
    res
}

pub fn next_collatz(n: u64) -> u64 {
    if n % 2 == 0 { n / 2 } else {3 * n + 1}
}

pub fn collatz(x:u64, memo: &std::collections::HashMap<u64, Vec<u64>>) -> Vec::<u64> {
    let mut vals : Vec<u64> = vec![];
    let mut curr = x;
    while curr > 1 {
        if let Some(v) = memo.get(&curr) {
            
            vals.append(&mut v.clone());
            //println!("found rest after {} in memo...", curr);
            return vals
        } else {
            vals.push(curr);
            //println!("calculating next after {}...", curr);
            curr = next_collatz(curr);
            
        }
    }
    vals.push(curr);
    vals
}

