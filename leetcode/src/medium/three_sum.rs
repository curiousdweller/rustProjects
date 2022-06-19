pub fn naive_three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // Brute force method run 3 loops, check all permutations.
    for (index, i) in nums.iter().enumerate() {
        for j in (index + 1)..nums.len() {
            for k in (index + 2)..nums.len() {
                println!("{}{}{}", nums[index], &nums[j], &nums[k]);
            }
        }
    }
    vec![]
}

pub fn sorted_three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    nums.sort();

    let mut previous = nums[0];
    for (index, i) in nums.iter().enumerate() {
        if index > nums.len() - 3 {
            break
        }
        if *i == previous && index > 0 {
            continue;
        }
        let mut left = index + 1;
        let mut right = nums.len() - 1;
        while left != right {
            println!("{}  {}  {}", index, left, right);
            if nums[index] + nums[left] + nums[right] > 0 {
                println!("{}", right);
                right -= 1;
            } else if  nums[index] + nums[left] + nums[right] < 0{
                left += 1;
            } else {
                res.push(vec![nums[index], nums[left], nums[right]]);
                break
            }
        }
        previous = nums[index];
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn second_test() {
        println!("{:?}", sorted_three_sum(vec![-3, 1, 2, -3, 4]));
    }
}