impl Solution {
    pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
        Self::merge_sort(&nums)
    }

    fn merge_sort(arr: &[i32]) -> Vec<i32> {
        if arr.len() <= 1 {
            return arr.to_vec();
        }

        let pivot = arr.len() / 2;

        let left = Self::merge_sort(&arr[..pivot]);
        let right = Self::merge_sort(&arr[pivot..]);

        let mut res = vec![];
        let mut l = 0;
        let mut r = 0;

        while l < left.len() && r < right.len() {
            if left[l] < right[r] {
                res.push(left[l]);
                l += 1;
            } else {
                res.push(right[r]);
                r += 1;
            }
        }

        while l < left.len() {
            res.push(left[l]);
            l += 1;
        }

        while r < right.len() {
            res.push(right[r]);
            r += 1;
        }

        res
    }
}
