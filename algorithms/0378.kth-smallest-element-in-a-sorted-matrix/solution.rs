use std::collections::BinaryHeap;

impl Solution {
    pub fn kth_smallest(matrix: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut heap = BinaryHeap::new();

        for row in matrix {
            for el in row {
                heap.push(el);

                if (k as usize) < heap.len() {
                    heap.pop();
                }
            }
        }

        heap.peek().unwrap().clone()
    }
}
