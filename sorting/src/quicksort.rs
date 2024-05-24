

use super::Sorter;

pub struct QuickSort;



fn quicksort<T: Ord>(slice: &mut [T]) {
    // Not in place
    // match slice.len() {
    //     0 | 1 => return,
    //     2 => {
    //         if slice[0] > slice[1] {
    //             slice.swap(0,1);
    //         }
    //         return;
    //     },
    //     _ => {}
    // }
    // let pivot = &slice[0];
    // let mut left = vec![];
    // let mut right = vec![];

    // for i in slice {
    //     if slice[i] <= pivot {
    //         left.push(slice[i]);
    //     } else {
    //         right.push(slice[i]);
    //     }
    // }
    // quicksort(left);
    // quicksort(right);

    // In Place
    match slice.len() {
        0 | 1 => return,
        2 => {
            if slice[0] > slice[1] {
                slice.swap(0,1);
            }
            return;
        },
        _ => {}
    };

    let (pivot, rest) = slice.split_first_mut().expect("Slice is non-empty"); // Allows to have a reference to a pivot and mutate the slice
    let mut left = 0;
    let mut right = rest.len() - 1;
    
    while left <= right {
        if  &rest[left] <= pivot { 
            left += 1;
        } else if &rest[right] > pivot {
            // right already in a correct place
            // avoid unnecessary swaps
            if right == 0 {
                break;
            }
            right -= 1;
        } else {
            rest.swap(left, right);
            left += 1;
            if right == 0 {
                break;
            }
            right -= 1;
        }
    }
    

    slice.swap(0, left);
    // need to exclude the pivot
    let (left, right) = slice.split_at_mut(left);
    quicksort(left);
    quicksort(&mut right[1..]);

}

impl Sorter for QuickSort {
    fn sort<T>(slice: &mut [T])
    where 
        T: Ord,
    {
        quicksort(slice);
    }
}

#[test]
fn quicksort_works() {
    let mut list = vec![4,2, 5,3,1, 84,7,10,6];
    super::sort::<_, QuickSort>(&mut list);
    println!("{:?} slice", list);
    assert_eq!(list, &[1,2,3,4,5,6,7,10,84]);
}
