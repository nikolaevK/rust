
    fn merge_sort(vec: &Vec<i32>) -> Vec<i32>   {
        // In Place
        match vec.len() {
            0 | 1 =>  vec.to_vec(),
            _ => {
    
                let length: usize = vec.len() / 2;
                let left = merge_sort(&vec[0..length].to_vec());
                let right = merge_sort(&vec[length..].to_vec());
            
                merge(left, right)
            }
    
        }
    }
    
    fn merge(left: Vec<i32>, right: Vec<i32>) -> Vec<i32> {
        let mut merged = vec![];
        let mut j = 0;
        let mut i = 0;
    
        while i < right.len() || j < left.len() { 
            if i >= right.len() {
                merged.push(left[j]);
                j += 1;
            } else if j >= left.len() {
                merged.push(right[i]);
                i += 1;
            } else if right[i] < left[j] {
                merged.push(right[i]);
                i += 1;
            } else {
                merged.push(left[j]);
                j += 1;
            }
        }
        merged
    } 


    
    

#[test]
fn merge_sort_works() {
    let list = vec![4,2, 5,3,1, 84,7,10,6];
    let result = merge_sort(&list);
    println!("{:?} slice", list);
    assert_eq!(result, &[1,2,3,4,5,6,7,10,84]);
}
