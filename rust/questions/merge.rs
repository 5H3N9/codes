impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_unstable_by_key(|v| v[0]);
        let mut ans: Vec<Vec<i32>> = Vec::new();
        for cur in intervals {
            match ans.last_mut() {
                Some(last) if cur[0] <= last[1] => {
                    last[1] = last[1].max(cur[1]);
                }
                _ => ans.push(cur),
            }
        }
        ans
    }
}
