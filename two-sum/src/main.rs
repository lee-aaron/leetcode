use std::collections::HashMap;

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("{:?}", result);
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if map.contains_key(&complement) {
            return vec![*map.get(&complement).unwrap() as i32, i as i32];
        }
        map.insert(num, i);
    }
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum() {
        let nums = vec![3,2,4];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![1,2]);
    }

    #[test]
    fn test_two_sum2() {
        let nums = vec![3,3];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0,1]);
    }
}