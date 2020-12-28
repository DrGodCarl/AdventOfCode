use anyhow::Result;

const START: usize = 109165;
const END: usize = 576723;

fn is_monotonic(numbers: &Vec<usize>) -> bool {
    numbers.iter().zip(numbers.iter().skip(1))
        .map(|(current, next)| current <= next)
        .all(|b| b)
}

fn valid(password: usize) -> bool {
    let digits: Vec<usize> = password.to_string()
        .chars()
        .map(|s| s.to_digit(10).unwrap() as usize)
        .collect();
    is_monotonic(&digits)
        && digits.iter().zip(digits.iter().skip(1))
        .any(|(f, s)| f == s)
}

fn valid_no_larger_group(password: usize) -> bool {
    let digits: Vec<usize> = password.to_string()
        .chars()
        .map(|s| s.to_digit(10).unwrap() as usize)
        .collect();
    let buf: [usize; 10] = [0; 10];
    is_monotonic(&digits)
        && digits.iter().fold(buf, |mut count_buffer: [usize; 10], next_number| {
        count_buffer[*next_number] += 1;
        count_buffer
    }).iter().any(|count| *count == 2)
}

#[test]
fn test_valid() {
    assert!(valid(111111));
    assert!(valid(123455));
    assert!(valid(1123459));
    assert!(!valid(123456));
    assert!(!valid(123454));
    assert!(!valid(323454));
    assert!(!valid(123450));
}

#[test]
fn test_valid_no_larger_group() {
    assert!(!valid_no_larger_group(12345));
    assert!(valid_no_larger_group(123455));
    assert!(valid_no_larger_group(112459));
    assert!(valid_no_larger_group(111122));
    assert!(!valid_no_larger_group(123444));
    assert!(!valid_no_larger_group(111458));
    assert!(!valid_no_larger_group(114583));
}

fn main() -> Result<()> {
    let result1 = (START..END).filter(|i| valid(*i)).count();
    println!("part 1: {}", result1);

    let result2 = (START..END).filter(|i| valid_no_larger_group(*i)).count();
    println!("part 2: {}", result2);

    Ok(())
}
