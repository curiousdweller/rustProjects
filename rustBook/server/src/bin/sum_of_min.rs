fn main() {
    fn sum_of_minimums(numbers: [[u8; 4]; 4]) -> u8 {
        let mut num = Vec::new();
        for mut list in numbers {
            list.sort();
            num.push(list[0]);
        }
        let mut res = 0;
        for numbers in &num {
            res += numbers;
        }
        println!("{:?}", num);
        res
    }
    println!("{}", sum_of_minimums([[7, 9, 8, 6], [6, 5, 4, 3], [5, 7, 4, 5], [7, 9, 4, 3]]));
}