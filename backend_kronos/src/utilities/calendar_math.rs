//! calendar_math.rs

use chrono::{Datelike, Local};

// Some utility methods to help with calculations around dates and times.

pub fn get_federal_fiscal_year() -> i32 {
    let today = Local::now().date_naive(); // Get today's date (local time)
    let year = today.year();
    let month = today.month();

    if month >= 10 {
        // Fiscal year starts in October
        year + 1
    } else {
        year
    }
}
