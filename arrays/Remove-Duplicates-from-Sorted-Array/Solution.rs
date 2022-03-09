impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut pre_index = 0;
        for i in (1..nums.len()) {
            if (nums[i] != nums[pre_index]) {
                pre_index += 1;
                nums[pre_index] = nums[i];
            }
        }

        (pre_index + 1) as i32
    }
}