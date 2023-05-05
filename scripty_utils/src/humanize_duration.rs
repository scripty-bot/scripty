use chrono::Duration;

pub fn humanize_duration(duration: Duration) -> String {
    let mut prev_set = false;
    let mut out = String::with_capacity(128);
    let week_count = duration.num_weeks();
    let day_count = duration.num_days() % 7;
    let hour_count = duration.num_hours() % 24;
    let minute_count = duration.num_minutes() % 60;
    let second_count = duration.num_seconds() % 60;

    fmt_unit!(out, week_count, "week", prev_set);
    fmt_unit!(out, day_count, "day", prev_set);
    fmt_unit!(out, hour_count, "hour", prev_set);
    fmt_unit!(out, minute_count, "minute", prev_set);
    fmt_unit!(out, second_count, "second", prev_set);
    out
}

macro_rules! fmt_unit {
    ($out:expr, $num:expr, $unit_name:expr, $pset:expr) => {
        #[allow(unused_assignments)]
        if $num != 0 {
            if $pset {
                $out.push_str(", ");
            }
            $out.push_str(&$num.to_string());
            if $num == 1 {
                $out.push_str(concat!(" ", $unit_name));
            } else {
                $out.push_str(concat!(" ", $unit_name, "s"));
            }
            $pset = true;
        }
    };
}
use fmt_unit;
