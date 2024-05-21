use super::Sorter;

pub struct SelectionSort;

impl Sorter for SelectionSort {
    fn sort<T>(slice: &mut [T])
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
    {
        for i in 0..slice.len() {
            let (min, _) = slice[i..]
            .iter()
            .enumerate() // converts the iterator into tuple of (index, value) 
            .min_by_key(|&(_, v)| v) // closure fishes min by the value, returns reference to an element
            .expect("slice is non-empty");
            let min = i + min;

            if i != min {
                slice.swap(i, min);
            }
        }
    }
}

#[test]
fn selection_works() {
    let mut list = vec![4,2, 5,3,1];
    super::sort::<_, SelectionSort>(&mut list);
    assert_eq!(list, &[1,2,3,4,5]);
}