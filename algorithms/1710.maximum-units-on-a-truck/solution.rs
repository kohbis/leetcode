impl Solution {
    pub fn maximum_units(box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        let mut res = 0;

        let mut box_types: Vec<Vec<i32>> = box_types;
        box_types.sort_by_key(|x| -x[1]);

        let mut truck_size = truck_size;
        for box_type in box_types {
            let (num, unit) = (box_type[0], box_type[1]);

            if num > truck_size {
                res += truck_size * unit;
                break;
            } else {
                res += num * unit;
                truck_size -= num
            }
        }

        res
    }
}
