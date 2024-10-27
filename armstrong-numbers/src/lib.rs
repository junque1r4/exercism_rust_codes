pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<u32> = num
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .collect();

    let num_digits = digits.len() as u32;

    let sum_of_powers: u32 = digits.iter().map(|&digit| digit.pow(num_digits)).sum();

    sum_of_powers == num
}
