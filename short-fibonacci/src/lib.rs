use std::vec;

/// Create an empty vector
pub fn create_empty() -> Vec<u8> {
    create_buffer(0)
}

/// Create a buffer of `count` zeroes.
///
/// Applications often use buffers when serializing data to send over the network.
pub fn create_buffer(count: usize) -> Vec<u8> {
    if count > 0 { let vector = vec![0; count]; vector } else { Vec::new() }
    }

/// Create a vector containing the first five elements of the Fibonacci sequence.
///
/// Fibonacci's sequence is the list of numbers where the next number is a sum of the previous two.
/// Its first five elements are `1, 1, 2, 3, 5`.
pub fn fibonacci() -> Vec<u8> {
    let mut fibonacci_vec: Vec<u8> = Vec::new(); 
    for number in 0..5 {
        if number == 0 || number == 1 { fibonacci_vec.push(1)} else {
        fibonacci_vec.push( fibonacci_vec[number-1] + fibonacci_vec[number-2]);
        }
    }

     fibonacci_vec
}
