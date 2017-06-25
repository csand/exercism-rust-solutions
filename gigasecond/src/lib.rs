extern crate chrono;
extern crate time;

use chrono::*;
use time::Duration;

const SECS_IN_GIGA: i64 = 1000000000;

pub fn after<Tz: TimeZone>(start_date: DateTime<Tz>) -> DateTime<Tz> {
    start_date + Duration::seconds(SECS_IN_GIGA)
}
