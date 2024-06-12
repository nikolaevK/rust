use std::{collections::HashSet, thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Progress<Iter> {
    iter: Iter,
    i: usize,
}

// Static method does not take self as an input
impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Progress {
            iter,
            i: 0
        }
    }
}

impl<Iter> Iterator for Progress<Iter> 
where 
    Iter: Iterator
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;
       self.iter.next()
    }
}

trait ProgressIteratorExtension: Sized {
    fn progress(self) -> Progress<Self>;
} 

impl<Iter> ProgressIteratorExtension for Iter {
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}
    
fn expensive_calculation() {
    sleep(Duration::from_secs(1));
}
fn main() {
    let v = vec![1, 2, 3,];

   for _n in v.iter().progress() {
        expensive_calculation();
   }
}
