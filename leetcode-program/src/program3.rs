
#[warn(dead_code)]
pub struct Solution{

}

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut len = 0;
        let mut buf_c  = "".to_string();
        for(_ ,c) in s.chars().enumerate() {
            match buf_c.find(c) {
                Some(i) => {
                    buf_c = buf_c[i+1..].to_string();
                    buf_c.push_str(c.to_string().as_str());
                    if max_len < len {
                        max_len = len;
                    }
                }
                None => {
                    buf_c.push_str(c.to_string().as_str());
                    len = buf_c.len();
                }
            }
        }

        if max_len < len {
            max_len = len;
        }

        return max_len as i32;
    }
}