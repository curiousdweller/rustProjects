pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut return_vec = vec![];
    'outer: for (index, num) in nums.iter().enumerate() {
        for i in 0..nums.len() {
            if i == index {
                continue;
            } else if target == num + nums[i] {
                return_vec.push(nums[i]);
                return_vec.push(*num);
                break 'outer;
            } else {
                continue;
            }
        }
    }

    return_vec
}
