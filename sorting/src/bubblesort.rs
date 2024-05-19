use super::Sorter;

pub struct Bubblesort;

impl Sorter for Bubblesort {
    fn sort<T>(slice: &mut [T])
    where
    T : Ord 
    {
        let mut swapped = true;
        while swapped {
            swapped = false;
            for i in 0..(slice.len() - 1) {
                if slice[i] > slice[i + 1] {
                    slice.swap(i, i  + 1);
                    swapped = true;
                }
            }
        }
    }
}

#[test]
fn bubblesort_works() {
    let mut list = vec![4,2, 5,3,1];
    super::sort::<_, Bubblesort>(&mut list);
    assert_eq!(list, &[1,2,3,4,5]);
}