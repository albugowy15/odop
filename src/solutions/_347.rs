use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use super::Solution;

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        if k == nums.len() as i32 {
            return nums;
        }

        let mut count_map: HashMap<i32, i32> = HashMap::new();
        for n in nums {
            // let count = count_map.get(&n).unwrap_or(&0);
            // count_map.insert(n, count + 1);
            *count_map.entry(n).or_insert(0) += 1;
        }

        let mut heap: BinaryHeap<(Reverse<i32>, i32)> = BinaryHeap::new();
        for (&num, &frequency) in count_map.iter() {
            heap.push((Reverse(frequency), num));
            if heap.len() > k as usize {
                heap.pop();
            }
        }

        let mut top: Vec<i32> = vec![0; k as usize];

        for i in (0..k).rev() {
            top[i as usize] = heap.pop().unwrap().1;
        }

        top
    }
}

#[test]
fn test_top_k_frequent_items() {
    // Test case 1
    let nums1 = vec![1, 1, 1, 2, 2, 3];
    let k1 = 2;
    let expected1 = vec![1, 2];
    let result1 = Solution::top_k_frequent(nums1, k1);
    assert_eq!(expected1, result1);

    // Test case 2
    let nums2 = vec![1];
    let k2 = 1;
    let expected2 = vec![1];
    let result2 = Solution::top_k_frequent(nums2, k2);
    assert_eq!(expected2, result2);
}
