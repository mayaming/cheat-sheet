pub struct Solution{}

impl Solution {
    pub fn jump(nums: Vec<i32>) -> i32 {
        let mut steps_2_end: Vec<i32> = Vec::new();
        let len = nums.len();
        for _i in 0..len {
            steps_2_end.push(i32::MAX);
        }
        steps_2_end[len-1] = 0;
        for i in 1..len {
            let idx = len - 1 - i;
            let val = nums[idx];
            for j in 1..(val+1) {
                if idx + (j as usize) < len {
                    steps_2_end[idx] = std::cmp::min(steps_2_end[idx], steps_2_end[idx + (j as usize)] + 1);
                }
            }
        }
        return steps_2_end[0];
    }
}