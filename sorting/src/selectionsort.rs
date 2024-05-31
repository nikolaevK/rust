use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(&self, slice: &mut [T])
    where
    T : Ord 
    // One Version
    // {
    //    // [sorted | not sorted]
    //     for i in 0..slice.len() {
    //         let mut min  = i;
    //         for j in ( i + 1)..slice.len() {
    //             if slice[j] < slice[min] {
    //                 min = j
    //             }
    //         }

    //         if i != min {
    //             slice.swap(i, min);
    //         }
    //     }
    // }
    // Second Version
    {
        for i in 0..slice.len() {
            let min = slice[i..]
            .iter()
            .enumerate() // converts the iterator into tuple of (index, value) 
            // min_by_key generates a reference to a tuple for the lifetime of the iteration then cleans it up
            .min_by_key(|&(_, v)| v) // closure fishes min by the value, returns reference to an element in the slice
            .map(|(index, _)| index + i) // Same as let min = i + min; // because slice starts at i, adjust the idex to account for the slicing of the slice
            .expect("slice is non-empty");
            

            if i != min {
                slice.swap(i, min);
            }
        }
    }
}

#[test]
fn selection_works() {
    let mut list = vec![4,2, 5,3,1];
    SelectionSort.sort(&mut list);
    assert_eq!(list, &[1,2,3,4,5]);
}