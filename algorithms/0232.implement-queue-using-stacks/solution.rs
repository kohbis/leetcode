use std::collections::VecDeque;

struct MyQueue {
    pub queue: VecDeque<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 * /**
 *  * Your MyQueue object will be instantiated and called as such:
 *  * let obj = MyQueue::new();
 *  * obj.push(x);
 *  * let ret_2: i32 = obj.pop();
 *  * let ret_3: i32 = obj.peek();
 *  * let ret_4: bool = obj.empty();
 *  */
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue {
            queue: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.queue.push_back(x)
    }

    fn pop(&mut self) -> i32 {
        if let Some(x) = self.queue.pop_front() {
            return x;
        }
        -1 // unreachable!()
    }

    fn peek(&self) -> i32 {
        if !self.empty() {
            return self.queue[0];
        }
        -1 // unreachable!()
    }

    fn empty(&self) -> bool {
        self.queue.len() == 0
    }
}
