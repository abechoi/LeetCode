struct Solution;

fn main() {
    
    let x = -121;

    println!("{:?}", Solution::is_palindrome(x));
}

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let str = x.to_string();

        for (i, c) in str.chars().enumerate() {

            let last = str.len() - 1 - i;
            let num = str.chars().nth(last).unwrap();
    
            if c != num {
                return false;
            }
            
        }
        true
    }
}