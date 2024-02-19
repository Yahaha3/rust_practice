
#[warn(dead_code)]
pub struct Solution{

}

impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        let n = s.len();
        let s = s.as_bytes();
        let mut dp = vec![vec![false; n]; n];
        let mut ans = &s[0..1];
        let mut max = 0;
        for j in 1..=n-1 {
            for i in 0..=j {
                if s[i] != s[j] {
                    dp[i][j] = false;
                } else {
                    if j as i32 - i as i32 - 1 <= 1 {
                        dp[i][j] = true;
                    } else {
                        dp[i][j] = dp[i+1][j-1];
                    }
                }

                if dp[i][j] && (j - i + 1 > max) {
                    max = j - i + 1;
                    ans = &s[i..i+max];
                }
            }
        }
    
        String::from_utf8(ans.to_vec()).unwrap()
    }
}