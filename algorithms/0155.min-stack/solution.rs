struct MinStack {
    stack: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self { stack: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val)
    }

    fn pop(&mut self) {
        if self.stack.len() > 0 {
            self.stack.pop().unwrap();
        }
    }

    fn top(&self) -> i32 {
        return self.stack[self.stack.len() - 1];
    }

    fn get_min(&self) -> i32 {
        self.stack.iter().min().unwrap().clone()
    }
}

// /**
//  * Your MinStack object will be instantiated and called as such:
//  * let obj = MinStack::new();
//  * obj.push(val);
//  * obj.pop();
//  * let ret_3: i32 = obj.top();
//  * let ret_4: i32 = obj.get_min();
//  */
