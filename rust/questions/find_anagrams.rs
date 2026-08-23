use std::collections::HashMap;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut pmp: HashMap<char, i32> = HashMap::new();
        let mut l = 0;
        let mut r = 0;
        let pl = p.len();
        let mut ans: Vec<i32> = Vec::new();
        let s: Vec<char> = s.chars().collect();
        for c in p.chars() {
            *pmp.entry(c).or_insert(0) += 1;
        }
        while r < s.len() {
            let c = s[r];
            *pmp.entry(c).or_insert(0) -= 1;
            while *pmp.get(&c).unwrap() < 0 {
                *pmp.get_mut(&s[l]).unwrap() += 1;
                l += 1;
            }
            if r - l == pl - 1 {
                ans.push(l as i32);
            }
            r += 1;
        }
        ans
    }
}
