struct Solution;
use std::collections::HashMap;

fn main() {

    let nums1: Vec<i32> = vec![2,7,11,15];
    let target1 = 9;
    let answer1 = Solution::two_sum(nums1, target1);

    let nums2: Vec<i32> = vec![3,2,4];
    let target2 = 6;
    let answer2 = Solution::two_sum(nums2, target2);

    let nums3: Vec<i32> = vec![3,3];
    let target3 = 6;
    let answer3 = Solution::two_sum(nums3, target3);

    let answers: [Vec<i32>; 3] = [answer1, answer2, answer3];

    for i in answers {
        println!("{:?}", i);
    }
}

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

        // Best Solution
        // let mut hm = HashMap::new();

        // for (i, &val) in nums.iter().enumerate() {

        //     let look = target - val;

        //     if let Some(&j) = hm.get(&look) {
        //         println!("i = {}, {:?} {:?}", i, Some(&j), hm.get(&look));
        //         return vec![i as i32, j];
        //     }

        //     hm.insert(val, i as i32);
        // }
        // vec![]

        // Optimzed Solution
        // let mut hm = HashMap::new();
        // for (i, &val) in nums.iter().enumerate() {
        //     hm.insert(val, i as i32);
        // }
        
        // for (key, value) in &hm {
        //     println!("{} {}", key, value);
        // }

        // for (i, &val) in nums.iter().enumerate() {
        //     let look = target - val;

        //     if let Some(&j) = hm.get(&look) {

        //         println!("Some(&j) = {:?}, &j = {:?}, hm.get(&look) = {:?}, look = {:?}", Some(&j), &j, hm.get(&look), look);
                
        //         let pos = j as usize;
        //         if i != pos {
        //             println!("i = {:?}, pos = {:?}", i, pos);
        //             return vec![i as i32, j];
        //         }
        //     }
        // }
        // vec![]
    }
}