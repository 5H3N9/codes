use std::collections::VecDeque;

impl Solution {
    pub fn max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;
        let nl = nums.len();
        let mut ans: Vec<i32> = Vec::with_capacity(nl - k + 1);
        let mut dq: VecDeque<usize> = VecDeque::new();
        for r in 0..nl {
            while let Some(&b) = dq.back() {
                if nums[r] > nums[b] {
                    dq.pop_back();
                } else {
                    break;
                }
            }
            dq.push_back(r);
            if dq[0] + k <= r {
                dq.pop_front();
            }
            if r >= k - 1 {
                ans.push(nums[dq[0]]);
            }
        }
        ans
    }
}
