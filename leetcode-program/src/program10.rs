#[warn(dead_code)]
pub struct Solution{

}

impl Solution {

    pub fn is_match(s: String, p: String) -> bool {

        let s = s.as_bytes();
        let p = p.as_bytes();

        if p[p.len() - 1] == '*' as u8 {
            return true
        } 

        let mut i: i32 = p.len() as i32 - 1;
        let mut j: i32 = s.len() as i32 - 1;
        let mut umatch = false;
        while i >=0 {
            if j < 0 {
                i-=1;
            }
            if umatch &&  s[j as usize] == p[i as usize]{
                j-=1;
            } 
            else if s[j as usize] == p[i as usize] || p[i as usize] == '.' as u8 {
                i-=1;
                j-=1;
            } else if p[i as usize] == '*' as u8 {
                i-=1;
                umatch = true;
            } else {
                i-=1;
                umatch = false;
            }
        }
        i == 0

    }

}