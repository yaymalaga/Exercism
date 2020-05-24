/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let code = code.replace(" ", "");

    if code.len() <= 1 {
        return false;
    }

    let mut total_sum = 0;
    for (index,value) in code.chars().rev().enumerate() {
        if !value.is_numeric() {
            return false;
        }

        let number = value.to_digit(10).unwrap();
        if index % 2 != 0 {
            if number * 2 > 9 {
                total_sum += number * 2 - 9;
            } else {
                total_sum += number * 2;
            }
        } else {
            total_sum += number;
        }
    }
    
    total_sum % 10 == 0
}
