use chrono::prelude::*;

/// Compute binary string for hours, minutes and seconds
pub fn get_time() -> [String; 6] {
    let dt = Local::now();
    
    let (h1, h2) = get_bin_string(dt.hour());
    let (m1, m2) = get_bin_string(dt.minute());
    let (s1, s2) = get_bin_string(dt.second());

    [h1, h2, m1, m2, s1, s2]
}

/// Compute binary string from 2 digit decimal
pub fn get_bin_string(n: u32) -> (String, String) {
    let left = n/10;
    let right = n - 10 *  left;
    (format!("{left:04b}"), format!("{right:04b}"))
}