use super::Sorter;

pub struct InsertionSort;

impl Sorter for InsertionSort {
    fn sort<T>(slice: &mut [T])
    where
    T : Ord 
    {
       // [sorted | not sorted]
        for unsorted in 1..slice.len() {

            let mut j  = unsorted;
            while j > 0 && slice[j - 1] > slice[j] {
                slice.swap(j - 1, j);
                j -= 1;
            }
        }

    }
}

#[test]
fn insertion_sort_works() {
    let mut list = vec![4,2, 5,3,1];
    super::sort::<_, InsertionSort>(&mut list);
    assert_eq!(list, &[1,2,3,4,5]);
}