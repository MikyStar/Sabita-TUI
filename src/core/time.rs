use std::time::Duration;

use humanize_duration::{prelude::DurationExt, Truncate};

////////////////////////////////////////

pub fn seconds_to_hr(time: Duration) -> String {
    time.human(Truncate::Second).to_string()
}
