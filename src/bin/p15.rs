use std::collections::vec_deque::{VecDeque, Iter};

fn main() -> () {
    let mut l = Lattice::new(2,2);
    let paths = l.find_paths();

    eprintln!("{}\n", paths.len());
    for p in paths.iter().take(10) {
        eprintln!("{}\n", p);
    }
}

#[derive(Debug)]
pub struct Lattice {
    height: usize,
    width: usize,
}

#[derive(Debug,Clone)]
pub struct LatticePath {
    path: Vec<PathStep>,
    path_height: usize,
    path_width: usize
}

#[derive(Debug,Clone,Copy,PartialEq,PartialOrd,Eq)]
pub enum PathStep {
    Right,
    Down
}

impl Lattice {
    pub fn new(height: usize, width: usize) -> Lattice {
        Lattice { height, width }
    }

    pub fn find_paths(&mut self) -> Vec<LatticePath> {
        let start = LatticePath::new();
        let mut procs = vec![start];
        let mut fin = vec![];
        while let Some(n) = procs.pop() {
            let maxdim = if n.path.len() == self.height { self.height } else {n.path_height + 1};
            let avail = avail_steps(maxdim, maxdim, &n);
            if n.path.len() + 1 > self.height && avail.len() == 0 {

                fin.push(n);

            } else {
                for s in avail {
                    let mut np = n.clone();
                    np.step(s);
                    procs.push(np);
                }
            }

            
        }

        fin
    }
}

impl LatticePath {
    pub fn step(&mut self, step: PathStep) -> () {
        match step {
            PathStep::Down => {
                self.path_height += 1;
                self.path.push(PathStep::Down);
            },
            PathStep::Right => {
                self.path_width += 1;
                self.path.push(PathStep::Right);
            }
        }
        ()
    }

    fn new() -> LatticePath {
        LatticePath { path_height: 0, path_width: 0, path: vec![]}
    }
}

fn avail_steps(max_height: usize, max_width : usize,  lpath: &LatticePath) -> Vec<PathStep> {
    let mut avail = vec![];
    if max_height > lpath.path_height{
        avail.push(PathStep::Down);
    } 
    if max_width > lpath.path_width  {
        avail.push(PathStep::Right);
    }
    avail
}

impl std::fmt::Display for LatticePath {
    
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        let mut curr_width: usize = 0;
        for i in &self.path {
            if i == &PathStep::Right {
                write!(f, "-> ")?;
                curr_width += 1;
            } else {
                write!(f, "\n")?;
                writeln!(f, "{:>width$}","|", width=curr_width*3 + 1)?;
                write!(f, "{:>width$}", "v",width=curr_width*3 + 1)?;

            }
        }

        Ok(())
     }
}