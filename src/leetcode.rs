/// https://leetcode.com/problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    use std::collections::HashMap;

    let mut seen: HashMap<i32, i32> = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        match seen.get(num) {
            None => seen.insert(target - num, index as i32),
            Some(&first_index) => return vec![first_index, index as i32],
        };
    }
    vec![]
}
