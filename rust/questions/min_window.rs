use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let s = s.as_bytes();
        let mut hp: HashMap<u8, i32> = HashMap::new();
        for &c in t.as_bytes() {
            *hp.entry(c).or_insert(0) += 1;
        }
        let mut l = 0;
        let mut m = hp.len();
        let mut ss = s.len();
        let mut sl = s.len() + 1;
        for (r, &c) in s.iter().enumerate() {
            if let Some(rc) = hp.get_mut(&c) {
                *rc -= 1;
                if *rc == 0 {
                    m -= 1;
                }
                if m == 0 {
                    while m == 0 {
                        if let Some(lc) = hp.get_mut(&s[l]) {
                            *lc += 1;
                            if *lc > 0 {
                                m += 1;
                            }
                        }
                        l += 1;
                    }
                    let nl = r + 2 - l;
                    if nl < sl {
                        sl = nl;
                        ss = l - 1;
                    }
                }
            }
        }
        if ss == s.len() {
            return "".to_string();
        }
        String::from_utf8_lossy(&s[ss..ss + sl]).to_string()
    }
}
