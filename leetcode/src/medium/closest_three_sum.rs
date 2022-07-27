pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    let mut closest = 0;
    nums.sort();
    let mut previous = 0;

    for (index, num) in nums.iter().enumerate() {
        println!("{}", index);
        let num = *num;
        if num == previous && index != 0 {
            continue;
        }
        if index > nums.len() - 3 {
            println!("Index : {}", index);
            break;
        }
        let mut left = index + 1;
        let mut right = nums.len() - 1;
        let mut total = nums[left] + nums[right] + num;
        if index == 0 {
            closest = total;
        }
        while left != right {
            total = nums[left] + nums[right] + num;
            // some func with 3 params, returns true/false and neg/pos diff:
            if total == target {
                return total
            }
                let new_diff = target - total;
                let closest_diff = target - closest;
                // if its negative, we should reduce right
                // if positive, then increase left
                match new_diff.is_negative() {
                    true => {
                        right -= 1;
                    }

                    false => {
                        left += 1;
                    }
                }
                if new_diff.abs() < closest_diff.abs() {
                    closest = total;
                } 

        }
        // optimise so that repeats are ignored.
        previous = num;
    }
        closest
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn second_test() {
        println!("test");
        assert_eq!(three_sum_closest(vec![-3, 1, 2, -3, 4], -4), -4);
        println!("test");
        assert_eq!(three_sum_closest(vec![-1,2,1,-4], 1),2);
        println!("test");
        assert_eq!(three_sum_closest(vec![0,0,0], 1),0);
        println!("test");
        assert_eq!(three_sum_closest(vec![1,1,1,1], 3),3);
        println!("test");
        assert_eq!(three_sum_closest(vec![1,1,-1,-1,3], 3),3);
        println!("test");
        assert_eq!(three_sum_closest(vec![0,2,1,-3], 1),0);
        println!("test");
    }
}