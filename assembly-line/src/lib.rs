// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let mut cars_produced: f64;
    let quantity_base: f64 = 221.;

    match speed {
        1 ..= 4 => speed as f64 * quantity_base,
        5 ..= 8 => speed as f64 * quantity_base / 100. * 90.,
        9 ..= 10 => speed as f64 * quantity_base / 100. * 77.,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let total_products = production_rate_per_hour(speed) / 60.;
    
    total_products as u32
}
