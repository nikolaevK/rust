use sorting::*;
use rand::prelude::*;
use std::cmp::Ordering;
use std::cell::Cell;
use std::rc::Rc;

#[derive(Clone)]
pub struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t
    }
}

impl <T:Eq> Eq for SortEvaluator<T> {}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
    
}

impl<T:Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("T: Ord")
    }
}




fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));
    for n in [0,1,10,100,1000,10000] {
        let mut values = Vec::with_capacity(n);
        let mut values_2 = Vec::with_capacity(n);
        for _ in 0..n {
            values.push(SortEvaluator {
                t: rand.gen::<usize>(),
                cmps: Rc::clone(&counter),
            });
            values_2.push(SortEvaluator {
                t: rand.gen::<i32>(),
                cmps: Rc::clone(&counter),
            })
        }

        for _ in 0..10 {
            values.shuffle(&mut rand);
            values_2.shuffle(&mut rand);

            let took = bench(BubbleSort, &values, &counter);
            println!("{} {} {}", "Bubble", n, took);
            let took = bench(InsertionSort ,&values, &counter);
            println!("{} {} {}", "Insertion", n, took);
            let took = bench(SelectionSort, &values, &counter);
            println!("{} {} {}", "Selection", n, took);
            let took = bench(QuickSort, &values, &counter);
            println!("{} {} {}", "Quicksort", n, took);
            // let took = bench_2(MergeSort, &values_2, &counter);
            // println!("{} {} {}", "MergeSort", n, took);
        }

    }
}

fn bench<T: Ord + Clone, S: Sorter>(sorter: S, values: &[SortEvaluator<T>], counter: &Cell<usize>) -> usize {
    let mut values:Vec<_> = values.to_vec();
    counter.set(0);
    sorter.sort(&mut values);
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1]);
    }
    counter.get()
}

// fn bench_2<S: MergeSortSorter>(sorter: S, values_2: &[SortEvaluator<i32>], counter: &Cell<usize>) -> usize {
//     let mut values: Vec<_> = values_2.to_vec();
//     counter.set(0);
//     sorter.sort(values);
//     for i in 1..values_2.len() {
//         assert!(values_2[i] >= values_2[i - 1]);
//     }
//     counter.get()
// }