use aoc_runner_derive::aoc_lib;

/// Random utility functions
pub mod util {
    use std::cmp::max;

    pub fn merge_sort<T>(array: &[T]) -> Vec<T> 
        where T: PartialEq + PartialOrd + Clone
    {
        // Handle base case:
        if array.len() == 1 { return vec![array[0].clone()]; }
        // if array.len() == 2 {
        //     if array[0] <= array[1] {
        //         return array.clone();
        //     }
        //     else {
        //         let mut out = Vec::with_capacity(2);
        //         out.push(array[1].clone());
        //         out.push(array[0].clone());
        //     }
        // }

        // Split array in two
        let split_index: usize = array.len() / 2;
        let (a, b) = array.split_at(split_index);

        // Sort each half
        let sorted_a = merge_sort(a);
        let sorted_b = merge_sort(b);

        // Merge the two halves
        return merge(&sorted_a, &sorted_b);
    }

    fn merge<T>(a: &[T], b: &[T]) -> Vec<T> 
        where T: PartialOrd + Clone
    {
        let mut out: Vec<T> = Vec::with_capacity(a.len() + b.len());

        let mut index_a = 0;
        let mut index_b = 0;
        loop {
            let item_a = a.get(index_a);
            let item_b = b.get(index_b);

            match (item_a, item_b) {
                (Some(item_a), Some(item_b)) => {
                    if item_a < item_b {
                        out.push(item_a.clone());
                        index_a += 1;
                    } else {
                        out.push(item_b.clone());
                        index_b += 1;
                    }
                }
                (Some(item_a), None) => {
                    out.push(item_a.clone());
                    index_a += 1;
                },
                (None, Some(item_b)) => {
                    out.push(item_b.clone());
                    index_b += 1;
                },
                (None, None) => break
            }            
        }

        out
    }
}

mod day1;

aoc_lib!{ year = 2024 }