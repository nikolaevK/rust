


pub trait Sorter {
    fn sort<T>(&self, slice: &mut [T])
    where
        T: Ord + Clone;
}

pub trait MergeSortSorter {
    fn sort(&self, slice: &Vec<i32>) -> Vec<i32>;
  
}



mod bubblesort;
mod insertionsort;
mod selectionsort;
mod quicksort;
mod mergesort;


pub use bubblesort::BubbleSort;
pub use insertionsort::InsertionSort;
pub use selectionsort::SelectionSort;
pub use quicksort::QuickSort;
pub use mergesort::MergeSort;

#[cfg(test)]
mod tests {
    use super::*;

    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(&self, slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort()
        }
    }

    #[test]
    fn std_works() {
        let mut list = vec![5,3,4,1,2];
        StdSorter.sort(&mut list);
        assert_eq!(list, &[1,2,3,4,5]);
    }
}
