// Summing multiples of 3 and 5 up to x
fn sum_multiples(x: i32) -> i32 {
    (0..=x).filter(|t| {t % 3 == 0 || t % 5 == 0}).sum()
}

fn main() {
    println!("Multiple sum is {}", sum_multiples(5));
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn test_multiples() {
        assert_eq!(sum_multiples(10), 33);
    }
}