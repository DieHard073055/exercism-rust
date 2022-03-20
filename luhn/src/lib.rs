fn sum_it_up(enumerate_tuple:(usize, u32)) -> u32{
    let (index, number) = enumerate_tuple;
    println!("index: {}, number {}", index, number);
    let mut return_value = number;
    if index % 2 == 0 {
        return_value = return_value * 2;
    }
    if return_value > 9 {
        return_value -= 9;
    }
    return_value
}
fn check_valid(cc_numbers: Vec<char>) -> bool{

    const RADIX: u32 = 10;
    let luhn_total: u32 = cc_numbers.iter()
        .map(|letter|letter.to_digit(RADIX).unwrap())
        .enumerate().map(sum_it_up).sum();

    println!("{:?}", luhn_total);
    luhn_total % 10 == 0
}
/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let cc_numbers: Vec<char> = code.chars().into_iter().filter(|c| !c.is_whitespace()).collect();
    if cc_numbers.len() <= 1 {
        return false;
    } 

    check_valid(cc_numbers.clone()) || check_valid(cc_numbers.clone().into_iter().rev().collect())
    
}
