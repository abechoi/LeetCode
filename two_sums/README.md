<div align="center">
<h1>Two Sums</h1>
<p>By Abe Choi</p>
</div>

<p align="center">
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.
</p>

1.  [Details](#details)
2.  [Brute Force Solution](#brute-force-solution)
3.  [Optimzed Solution](#optimized-solution)
4.  [Chapter IV](#chapter-iv)
5.  [Chapter V](#chapter-v)
6.  [Chapter VI](#chapter-vi)
7.  [Chapter VII](#chapter-vii)
8.  [Chapter VIII](#chapter-viii)
9.  [Chapter IX](#chapter-ix)
10. [Chapter X](#chapter-x)


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

```
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        // 1. Iterate through the Vector
        for (i, &a) in nums.iter().enumerate() {

            // 2. Iterate through the Vector (nested)
            for (j, &b) in nums.iter().enumerate() {

                // 3. Create an if statement
                // "i != j" is ensures two different indices
                // "a + b == target" checks to ensure the values add up to the target
                if i != j && a + b == target {

                    // 4. Return a Vector with the solution
                    return vec![i as i32, j as i32];
                }
            }    
        }
        // 5. Return empty Vector if solution is not found
        return vec![]
    }
}
```

## Optimzed Solution

2(n)

```
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        // 1. Create a HashMap
        let mut hm = HashMap::new();

        // 2. Add the values to the Hashmap
        for (i, &val) in nums.iter().enumerate() {
            hm.insert(val, i as i32);
        }

        // 3. Iterate through the Vector
        for (i, &val) in nums.iter().enumerate() {

            // 4. Create a variable to look for
            let look = target - val;


            if let Some(&j) = hm.get(&look) {
                let pos = j as usize;
                if i != pos {
                    return vec![i as i32, j];
                }
            }
        }
        return vec![];
    }
}
```

## Best Solution

O(n)

```
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        // 1. Create a HashMap
        let mut hm = HashMap::new();

        // 2. Iterate through the Vector
        for (i, &val) in nums.iter().enumerate() {

            // 3. Create a variable to look for
            let look = target - val;
            if let Some(&j) = hm.get(&look) {
                return vec![i as i32, j];
            }
            hm.insert(val, i as i32);
        }
        return vec![];
    }
}
```

## Chapter V

Enter information about Chapter V.

```
# this ia a code block for chapter v.
```

## Chapter VI

Enter information about Chapter VI.

```
# this ia a code block for chapter vi.
```

## Chapter VII

Enter information about Chapter VII.

```
# this ia a code block for chapter vii.
```
## Chapter VIII

Enter information about Chapter VIII.

```
# this ia a code block for chapter viii.
```

## Chapter IX

Enter information about Chapter IX.

```
# this ia a code block for chapter ix.
```

## Chapter X

Enter information about Chapter X.

```
# this ia a code block for chapter x.
```