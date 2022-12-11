fn main() {
    println!("{}", palindrome_number(121));
}

fn palindrome_number(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }
    let mut x = x;
    let mut rev = 0;
    while x > rev {
        rev = rev * 10 + x % 10;
        x /= 10;
    }
    rev == x || rev / 10 == x
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_palindrome_number() {
        assert!(palindrome_number(121));
        assert!(!palindrome_number(-121));
        assert!(!palindrome_number(10));
    }
}
