pub fn fizzbuzz() -> Vec<String> {
    (1..=100)
        .map(|i| {
            if i % 15 == 0 {
                "FizzBuzz".to_string()
            } else if i % 3 == 0 {
                "Fizz".to_string()
            } else if i % 5 == 0 {
                "Buzz".to_string()
            } else {
                i.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizzbuzz() {
        let result = fizzbuzz();
        assert_eq!(result.len(), 100);
        assert_eq!(result[0], "1");
        assert_eq!(result[2], "Fizz");
        assert_eq!(result[4], "Buzz");
        assert_eq!(result[14], "FizzBuzz");
    }
}
