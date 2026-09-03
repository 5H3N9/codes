impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = nums[0];
        let mut cur = nums[0];
        for i in 1..nums.len() {
            cur = nums[i].max(cur+nums[i]);
            max = max.max(cur);
        }
        max
    }
}
