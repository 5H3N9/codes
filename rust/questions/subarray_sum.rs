use std::collections::HashMap;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut mp: HashMap<i32, i32> = HashMap::new();
        let mut ans = 0;
        let mut ps = 0;
        *mp.entry(ps).or_insert(0) += 1;
        for p in 0..nums.len() {
            ps += nums[p];
            if let Some(&c) = mp.get(&(ps - k)) {
                ans += c;
            }
            *mp.entry(ps).or_insert(0) += 1;
        }
        ans
    }
}
