//! Module for duration humanisation. See [`humanise_duration_ms`] and [`humanise_duration`].

use crate::{humanise_list, plural_suffix};
use std::time::Duration;

const SECOND: u128 = 1000;
const MINUTE: u128 = SECOND * 60;
const HOUR: u128 = MINUTE * 60;
const DAY: u128 = HOUR * 24;

/// Humanise a duration specified in milliseconds.
///
/// # Arguments
///
/// * `milliseconds`: The total amount of milliseconds in the duration.
/// * `verbose`: Whether or not to be verbose or shorten a few of the longer words.
///              This only changes the words used for minutes (`minutes` becomes `mins`),
///              seconds (`seconds` becomes `secs`), and milliseconds (`milliseconds` becomes `ms`.
///
/// # Return value
///
/// The duration, humanised.
/// If `milliseconds` is zero, it defaults to `0 seconds` (or `0 secs` - see `verbose`).
///
/// # Examples
///
/// ```
/// use humanise::{humanise_duration_ms};
///
/// // The parameter is specified in milliseconds (if it wasn't already quite clear) - 123 milliseconds is just 123 milliseconds.
/// assert_eq!(humanise_duration_ms(123, true), "123 milliseconds");
/// // One second is equal to 1000 milliseconds, and this function uses the largest units first.
/// assert_eq!(humanise_duration_ms(1000, true), "1 second");
/// // When they are combined, we get nice and pretty output like so
/// assert_eq!(humanise_duration_ms(1234, true), "1 second and 234 milliseconds");
/// assert_eq!(humanise_duration_ms(62345, true), "1 minute, 2 seconds, and 345 milliseconds");
/// // Note that humanise_duration_ms only supports units up to days - after that, the units aren't fixed which creates problems.
/// assert_eq!(humanise_duration_ms(36 * 24 * 60 * 60 * 1000, true), "36 days");
/// ```
pub fn humanise_duration_ms(milliseconds: u128, verbose: bool) -> String {
    if milliseconds == 0 {
        return if verbose {
            "0 seconds".to_string()
        } else {
            "0 secs".to_string()
        };
    }
    let days_mod = milliseconds % DAY;
    let days = (milliseconds - days_mod) / DAY;
    let remaining_millis = days_mod;

    let hours_mod = remaining_millis % HOUR;
    let hours = (remaining_millis - hours_mod) / HOUR;
    let remaining_millis = hours_mod;

    let minutes_mod = remaining_millis % MINUTE;
    let minutes = (remaining_millis - minutes_mod) / MINUTE;
    let remaining_millis = minutes_mod;

    let seconds_mod = remaining_millis % SECOND;
    let seconds = (remaining_millis - seconds_mod) / SECOND;

    let milliseconds = seconds_mod;

    let mut vec = vec![];
    if days > 0 {
        vec.push(format!("{} {}", days, plural_suffix(days, "day", false)));
    }
    if hours > 0 {
        vec.push(format!("{} {}", hours, plural_suffix(hours, "hour", false)));
    }
    if minutes > 0 {
        vec.push(format!(
            "{} {}",
            minutes,
            plural_suffix(minutes, if verbose { "minute" } else { "min " }, false)
        ));
    }
    if seconds > 0 {
        vec.push(format!(
            "{} {}",
            seconds,
            plural_suffix(seconds, if verbose { "second" } else { "sec" }, false)
        ));
    }
    if milliseconds > 0 {
        vec.push(format!(
            "{} {}",
            milliseconds,
            if verbose {
                plural_suffix(milliseconds, "millisecond", false)
            } else {
                "ms".to_string()
            }
        ));
    }
    humanise_list(&vec)
}

/// Converts `duration` to milliseconds, then humanises that.
///
/// See [`humanise_duration_ms`].
pub fn humanise_duration(duration: Duration, verbose: bool) -> String {
    humanise_duration_ms(duration.as_millis(), verbose)
}

#[cfg_attr(docsrs, doc(cfg(feature = "chrono")))]
#[cfg(feature = "chrono")]
/// Converts `duration` to milliseconds, then humanises that.
///
/// See [`humanise_duration_ms`].
pub fn humanise_duration_chrono(duration: chrono::Duration, verbose: bool) -> String {
    humanise_duration_ms(duration.num_milliseconds().abs() as u128, verbose)
}

#[cfg(test)]
mod tests {
    use super::*;

    const fn duration(millis: u128) -> Duration {
        Duration::from_millis(millis as u64)
    }

    #[test]
    fn single_units_verbose() {
        macro_rules! test {
            ($duration:expr => $noun:literal, $max_safe:literal) => {
                assert_eq!(
                    humanise_duration(duration($duration * 1), true),
                    concat!("1 ", $noun)
                );
                assert_eq!(
                    humanise_duration(duration($duration * 2), true),
                    concat!("2 ", $noun, "s")
                );
                assert_eq!(
                    humanise_duration(duration($duration * $max_safe), true),
                    concat!($max_safe, " ", $noun, "s")
                );
            };
        }
        test!(DAY => "day", 128);
        test!(HOUR => "hour", 23);
        test!(MINUTE => "minute", 59);
        test!(SECOND => "second", 59);
        test!(1 => "millisecond", 999);
    }

    #[test]
    fn multiple_units_verbose() {
        assert_eq!(
            humanise_duration(duration(DAY + HOUR), true),
            "1 day and 1 hour"
        );
        assert_eq!(
            humanise_duration(duration(DAY + MINUTE), true),
            "1 day and 1 minute"
        );
        assert_eq!(
            humanise_duration(duration(3 * DAY + 7 * HOUR), true),
            "3 days and 7 hours"
        );
    }
}
