use std::convert::TryInto;

pub fn primes_less_than_n(n:u64) -> Vec<u64> {
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