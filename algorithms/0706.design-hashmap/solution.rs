use std::collections::HashMap;

#[derive(Debug)]
struct MyHashMap {
    datas: HashMap<i32, i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {
    fn new() -> Self {
        Self {
            datas: HashMap::new(),
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        *(self.datas).entry(key).or_default() = value;
    }

    fn get(&self, key: i32) -> i32 {
        if let Some(data) = self.datas.get(&key) {
            *data
        } else {
            -1
        }
    }

    fn remove(&mut self, key: i32) {
        *(self.datas).entry(key).or_default() = -1;
    }
}

// /**
//  * Your MyHashMap object will be instantiated and called as such:
//  * let obj = MyHashMap::new();
//  * obj.put(key, value);
//  * let ret_2: i32 = obj.get(key);
//  * obj.remove(key);
//  */
