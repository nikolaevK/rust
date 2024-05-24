

// Like an interface 
pub trait Sorter {
    fn sort<T>(slice: &mut [T])
    where
        T: Ord;
}

pub fn sort<T, S>(slice: &mut [T])
where
    T: Ord,
    S: Sorter,
{
    S::sort(slice)
}

mod bubblesort;
mod insertionsort;
mod selectionsort;
mod quicksort;

mod mergesort;


#[cfg(test)]
mod tests {
    use super::*;

    struct StdSorter;
    impl Sorter for StdSorter {
        fn sort<T>(slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort()
        }
    }

    #[test]
    fn std_works() {
        let mut list = vec![5,3,4,1,2];
        sort::<_, StdSorter>(&mut list);
        assert_eq!(list, &[1,2,3,4,5]);
    }
}
