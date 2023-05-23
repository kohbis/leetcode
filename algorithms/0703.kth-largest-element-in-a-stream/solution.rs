use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct KthLargest {
    size: usize,
    heap: BinaryHeap<Reverse<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl KthLargest {

    fn new(k: i32, nums: Vec<i32>) -> Self {
        let mut _self = KthLargest {
            size: k as usize,
            heap: BinaryHeap::new(),
        };
        for n in nums {
            _self.add(n);
        }
        _self
    }

    fn add(&mut self, val: i32) -> i32 {
        self.heap.push(Reverse(val));
        if self.heap.len() > self.size {
            self.heap.pop();
        }
        if let Some(Reverse(x)) = self.heap.peek() {
            *x
        } else {
            unreachable!()
        }
    }
}

/**
 * Your KthLargest object will be instantiated and called as such:
 * let obj = KthLargest::new(k, nums);
 * let ret_1: i32 = obj.add(val);
 */
