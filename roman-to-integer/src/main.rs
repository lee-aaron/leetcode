use std::collections::HashMap;

fn main() {
    println!("{}", roman_to_integer("III".to_string()));
}

fn roman_to_integer(s: String) -> i32 {
    let mut map = HashMap::new();
    map.insert('I', 1);
    map.insert('V', 5);
    map.insert('X', 10);
    map.insert('L', 50);
    map.insert('C', 100);
    map.insert('D', 500);
    map.insert('M', 1000);

    let mut result = 0;
    for i in 0..s.len() {
        let c = s.chars().nth(i).unwrap();
        if i + 1 < s.len() {
            let next_c = s.chars().nth(i + 1).unwrap();
            if map.get(&c).unwrap() < map.get(&next_c).unwrap() {
                result -= map.get(&c).unwrap();
                continue;
            }
        }
        result += map.get(&c).unwrap();
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roman_to_integer() {
        assert_eq!(roman_to_integer("III".to_string()), 3);
        assert_eq!(roman_to_integer("LVIII".to_string()), 58);
        assert_eq!(roman_to_integer("MCMXCIV".to_string()), 1994);
    }
}
