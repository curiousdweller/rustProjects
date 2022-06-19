use std::cmp;
pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = height.iter();
        let mut right = height.iter().rev();
        let mut width= (height.len() as i32) - 1;
        let mut l = left.next().unwrap();
        let mut r = right.next().unwrap();

        while width > 0 {
            let area = cmp::min(l, r) * width;
            max_area = cmp::max(area, max_area);

            if l > r {
                r = right.next().unwrap();
            } else {
                l = left.next().unwrap();
            }

            width -= 1;
        }
        max_area
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn max_area_test() {
        assert_eq!(max_area(vec![1,8,6,2,5,4,8,3,7]), 49);
    }
}