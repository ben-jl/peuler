use std::iter::FromIterator;
use std::convert::TryInto;

pub fn main() -> std::io::Result<()> {
    println!("p18");

    //let fpath = std::path::Path::new("./inputs/p18.txt");
    //let ns = load_input(&fpath)?;
    let ns = vec![3,7,4,2,4,6,8,5,9,3];
    const ROWS : usize = 4;
    let mut t : SumTriangle<ROWS> = SumTriangle::new(&ns[..]);

    
    for r in t.entries {
        for n in r {
            if n != 0 {
                print!("{: >2} ", n);
            }
            
        }
        print!("\n");
    }
    let res = t.find_traversal_with_max_sum();
    let mut sum = 0;
    for (rel_ix, val) in res {
        sum += val;
        println!("{:?}", &rel_ix);
    }
    println!("= {}", sum);
    //println!("{:?}", &res);
    //let sum = t.sum_traversal();
    //println!("{:?}", &sum);

    Ok(())
}

#[derive(Debug)]
pub struct SumTriangle<const B : usize> {
    entries: [[u32 ; B ] ; B],
    max_ix: usize,
    search_start: usize,
    search_end : usize
}

impl <const B : usize> SumTriangle<B> {
    pub fn new(vals: &[u32]) -> SumTriangle<B> {
        let siz : u32 = B.try_into().unwrap();
        let el_count : u32 = siz * (siz + 1) / 2;
        let val_count : u32 = vals.len().try_into().unwrap();
        if val_count > el_count {
            panic!("too many els {} {}", val_count, el_count);
        }

        let mut curr_ix : usize = 0;
        let mut entries = [[0 ; B] ; B];
        for i in 1..=B {
            for j in 1..=i {
                entries[i - 1][j - 1] = vals[curr_ix];
                curr_ix += 1;
            }
        }

        SumTriangle { entries, max_ix: curr_ix, search_start: 0, search_end: 0}
    }

    pub fn find_traversal_with_max_sum(&mut self) -> [(u32,u32) ; B] {
        let mut traversal_ixes = [(0 as u32, 0 as u32); B];

        
        for j in 0..B {
            let mut curr_max_val = 0;
            let mut curr_ix = 0;

            for i in self.search_start..=self.search_end {
                if self.entries[j][i] > curr_max_val {
                    curr_max_val = self.entries[j][i];
                    curr_ix = i;
                }
            }
            traversal_ixes[j] = (curr_ix.try_into().unwrap(), curr_max_val);
            self.search_start = curr_ix;
            self.search_end = curr_ix + 1;
        }
        traversal_ixes
    }

    pub fn sum_traversal(&mut self) -> u32 {
        let trs = self.find_traversal_with_max_sum();
        let mut sum = 0;
        for (_, v) in trs {
            sum += v;
        }
        sum
    }
}


fn load_input(path: &std::path::Path) -> std::io::Result<Vec<u32>> {
    let txt = std::fs::read_to_string(path)?;

    let mut nums : Vec<u32> = Vec::new();

    for raw_n in txt.split_ascii_whitespace() {
        let n : u32 = raw_n.parse().unwrap();
        nums.push(n);
    }

    Ok(nums)
}