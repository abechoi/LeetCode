<div align="center">
<h1>Two Sums</h1>
<p>By Abe Choi</p>
</div>

<p>
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.
</p>

1.  [Details](#details)
2.  [Brute Force Solution](#brute-force-solution)
3.  [Optimized Solution](#optimized-solution)
4.  [Best Solution](#best-solution)
5.  [Notable Lines](#notable-lines)
6.  [References](#references)


## Details

Example 1
```
Input: nums = [2,7,11,15], target = 9
Output: [0,1]
Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
```

Example 2
```
Input: nums = [3,2,4], target = 6
Output: [1,2]
```

Example 3
```
Input: nums = [3,3], target = 6
Output: [0,1]
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

O(n<sub>2</sub>)

1. Use a nested for loop.
2. Create an if statement:
   a. `i` and `j` are the indices, `a` and `b` are the values.
   b. Check to ensure `i != j`, and `a + b == target`.
3. Return correct indices (`i` and `j`) as a Vector of i32.

```
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        for (i, &a) in nums.iter().enumerate() {
            for (j, &b) in nums.iter().enumerate() {
                if i != j && a + b == target {
                    return vec![i as i32, j as i32];
                }
            }    
        }
        vec![]
    }
}
```

## Optimized Solution

2(n) => O(n)

1. Use a for loop to create a HashMap of the Vector.
2. Use another for loop to check the Vector.
3. Create an if statement:
   a. `look ` is the difference between `target and value`.
   b. `hm.get(&look)` returns a reference of `look` as an `Option` type.
   c. `if let Some(&j) = hm.get(&look)` matches the reference of `look` to the reference of `j`.
4. Create another if statement within the previous:
   a. Check to ensure each element was only used once.
   b. Return correct indices (`i` and `j`) as a Vector of i32.

```
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut hm = HashMap::new();

        for (i, &val) in nums.iter().enumerate() {
            hm.insert(val, i as i32);
        }

        for (i, &val) in nums.iter().enumerate() {

            let look = target - val;

            if let Some(&j) = hm.get(&look) {
                let pos = j as usize;
                if i != pos {
                    return vec![i as i32, j];
                }
            }
        }
        vec![]
    }
}
```

## Best Solution

O(n)

1. Use a for loop.
2. Create an if statement:
   a. `look ` is the difference between `target and value`.
   b. `hm.get(&look)` returns a reference of `look` as an `Option` type.
   c. `if let Some(&j) = hm.get(&look)` matches the reference of `look` to the reference of `j`.
3. Insert current `val` and `i as i32` into HashMap.

```
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut hm = HashMap::new();

        for (i, &val) in nums.iter().enumerate() {

            let look = target - val;
            if let Some(&j) = hm.get(&look) {
                return vec![i as i32, j];
            }
            hm.insert(val, i as i32);
        }
        vec![]
    }
}
```

## Notable Lines

1. `if let Some(&j) = hm.get(&look)` this line is looking for the index of look. First pass will fail since `hm` is empty.
2. `hm.insert(val, i as i32)` is at the end of the loop instead of the beginning because if the target is double the first element, `look` will return the first element which would use the same element twice.

## References

1. Examples of `if let` and `Some()`:
(https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
2. HashMaps:
(https://doc.rust-lang.org/std/collections/struct.HashMap.html#method.get)