struct OrderedStream {
    stream: Vec<String>,
    ptr: usize,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl OrderedStream {

    fn new(n: i32) -> Self {
        OrderedStream {
            stream: vec!["".to_string(); n as usize],
            ptr: 0,
        }
    }

    fn insert(&mut self, id: i32, value: String) -> Vec<String> {
        let mut chunk_list: Vec<String> = vec![];
        self.stream[(id-1) as usize] = value;

        while self.ptr < self.stream.len() && self.stream[self.ptr] != "" {
            chunk_list.push(self.stream[self.ptr].clone());
            self.ptr += 1
        }

        chunk_list
    }
}

/**
 * Your OrderedStream object will be instantiated and called as such:
 * let obj = OrderedStream::new(n);
 * let ret_1: Vec<String> = obj.insert(id, value);
 */
