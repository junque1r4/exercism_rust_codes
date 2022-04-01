// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

// First problem
pub fn expected_minutes_in_oven() -> i32 {
    let minutes_in_oven: i32 = 40;

    minutes_in_oven
}

// Second Problem
pub fn remaining_minutes_in_oven(actual_minutes_in_oven: i32) -> i32 {
    let remaining_minutes_in_oven: i32 = expected_minutes_in_oven() - actual_minutes_in_oven;

    remaining_minutes_in_oven
}

// Third problem
pub fn preparation_time_in_minutes(number_of_layers: i32) -> i32 {
    let preparation_time_in_minutes: i32 = number_of_layers * 2;

    preparation_time_in_minutes
}

// Last Problem
pub fn elapsed_time_in_minutes(number_of_layers: i32, actual_minutes_in_oven: i32) -> i32 {
    let elapsed_time_in_minutes: i32 = preparation_time_in_minutes(number_of_layers) + actual_minutes_in_oven;

    elapsed_time_in_minutes
}
