<div align="center">
<h1>Add Two Numbers</h1>
<p>By Abe Choi</p>
</div>

<p>
Given an integer x, return true if x is palindrome integer.

An integer is a palindrome when it reads the same backward as forward.

For example, 121 is a palindrome while 123 is not.
</p>

1.  [Details](#details)
2.  [Brute Force Solution](#brute-force-solution)
3.  [Notable Lines](#notable-lines)
4.  [References](#references)


## Details

Example 1
```
Input: x = 121
Output: true
Explanation: 121 reads as 121 from left to right and from right to left.
```

Example 2
```
Input: x = -121
Output: false
Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
```

Example 3
```
Input: x = 10
Output: false
Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
```

Constraints
```
- 2 <= nums.length <= 104
- 109 <= nums[i] <= 109
- 109 <= target <= 109
- Only one valid answer exists.
```

Follow-up
```
Can you come up with an algorithm that is less than O(n<sub>2</sub>) time complexity?
```

## Brute Force Solution

O(n)

1. Convert `x` to a String.
2. Create a for loop:
   a. `last` is the length of x minus 1 minus `i`. 
   b. `num` is the char of the `last` index (counting down).
3. Create an if statement:
   a. Check to ensure `c` equals `num`, else return false.

```
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
```

## Notable Lines

1. `str.chars().nth(last).unwrap()` returns a char from str of the `nth()` position.
2. `str.chars().nth(last).unwrap().to_digit(10).unwrap()` returns a base<sub>10</sub> number converted from the char.
2. `hm.insert(val, i as i32)` is at the end of the loop instead of the beginning because if the target is double the first element, `look` will return the first element which would use the same element twice.

## References

1. Get the nth Char in String:
https://www.codegrepper.com/code-examples/whatever/rust+get+nth+char+in+string
2. How to convert a Rust char to an integer so that '1' becomes 1?:
https://stackoverflow.com/questions/43983414/how-to-convert-a-rust-char-to-an-integer-so-that-1-becomes-1