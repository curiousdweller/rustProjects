pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut res = vec![];
    nums.sort();
    nums.dedup();


    for i in 0..=(nums.len() - 4) {

        for j in (i + 1)..=(nums.len() - 3) {
            println!("I: {} J: {}", i, j);
            let mut left = j + 1;
            let mut right = nums.len() - 1;


            while left != right {

                let temp = nums[i] + nums[j] + nums[left] + nums[right];
                println!("Temp: {}", temp);
                // Need to make bigger
                if temp < target {
                    left += 1;
                // Need to make smaller
                }
                if temp > target {
                    right -= 1;
                }
                 if temp == target {
                    res.push(vec![left as i32, right as i32, i as i32 , j as i32]);
                    left += 1;
                }
            }
        }
    }
    res
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn four_sum_test() {
        assert_eq!(four_sum(vec![1,0,-1,0,-2,2], 0).sort(), vec![[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]].sort())
    }
}