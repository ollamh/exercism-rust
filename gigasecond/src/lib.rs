extern crate chrono;
use chrono::prelude::*;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    let base: i64 = 10;
    let gigasecond = chrono::Duration::seconds(base.pow(9));
    return start + gigasecond;
}
