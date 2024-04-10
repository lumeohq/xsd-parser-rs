use std::convert::TryFrom;
use std::fmt;
use std::fmt::Write;
use std::str::FromStr;
use xsd_macro_utils::UtilsDefaultSerde;

#[derive(Default, PartialEq, PartialOrd, Debug, UtilsDefaultSerde)]
pub struct Duration {
    pub is_negative: bool,

    pub years: u64,
    pub months: u64,
    pub days: u64,

    pub hours: u64,
    pub minutes: u64,
    pub seconds: f64,
}

impl Duration {
    pub fn to_std_duration(&self) -> Result<std::time::Duration, String> {
        if self.years > 0 || self.months > 0 {
            Err("Duration with months or years require a starting date to be converted".into())
        } else {
            let secs = self.seconds as u64;

            let nanos = ((self.seconds - secs as f64) * 1e9) as u32;
            let secs = secs + 60 * (self.minutes + 60 * (self.hours + 24 * self.days));

            Ok(std::time::Duration::new(secs, nanos))
        }
    }

    pub fn from_std_duration(other: std::time::Duration) -> Result<Self, String> {
        const NANOS_PER_SEC: u128 = 1_000_000_000;
        let nanos = other.as_nanos() - other.as_secs() as u128 * NANOS_PER_SEC;

        let mut rest_value = other.as_secs();
        let seconds = (rest_value % 60) as f64 + nanos as f64 / 1e9;
        rest_value /= 60;
        let minutes = rest_value % 60;
        rest_value /= 60;
        let hours = rest_value % 24;
        rest_value /= 24;
        let days = rest_value;

        if days > 28 {
            return Err("Duration longer than 28 days is undefined".into());
        }

        Ok(Self {
            days,
            hours,
            minutes,
            seconds,
            ..Default::default()
        })
    }

    // TODO: Add a version of to_std_duration that takes a moment at time to start from.

    // TODO: Implement normalization function that takes a moment at time to start from and
    // converts months & years to days.
}

impl TryFrom<std::time::Duration> for Duration {
    type Error = String;
    fn try_from(value: std::time::Duration) -> Result<Self, Self::Error> {
        Duration::from_std_duration(value)
    }
}

impl TryFrom<Duration> for std::time::Duration {
    type Error = String;
    fn try_from(value: Duration) -> Result<Self, Self::Error> {
        value.to_std_duration()
    }
}

impl FromStr for Duration {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn fill_component(
            context: &mut ParsingContext,
            component: &mut u64,
            idx: i32,
            name: &str,
            symbol: char,
        ) -> Result<(), String> {
            if context.is_number_empty {
                return Err(format!(
                    "No value is specified for {}, so '{}' must not be present",
                    name, symbol
                ));
            }

            if context.is_dot_found {
                return Err("Only the seconds can be expressed as a decimal".into());
            }

            if context.last_filled_component >= idx {
                return Err("Bad order of duration components".into());
            }

            *component = context.number;
            context.last_filled_component = idx;
            context.number = 0;
            context.is_number_empty = true;

            Ok(())
        }

        fn fill_seconds(
            context: &mut ParsingContext,
            seconds: &mut f64,
        ) -> Result<(), &'static str> {
            if context.is_number_empty {
                return Err("No value is specified for seconds, so 'S' must not be present");
            }

            if context.is_dot_found && context.denom == 1 {
                return Err("At least one digit must follow the decimal point if it appears");
            }

            if context.last_filled_component >= 6 {
                return Err("Bad order of duration components");
            }

            *seconds = context.number as f64 + context.numer as f64 / context.denom as f64;
            context.last_filled_component = 6;
            context.number = 0;
            context.is_number_empty = true;

            Ok(())
        }

        let mut dur: Duration = Default::default();
        let mut context = ParsingContext::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                '-' => {
                    if i == 0 {
                        dur.is_negative = true;
                    } else {
                        return Err("The minus sign must appear first".into());
                    }
                }
                'P' => {
                    if i == 0 || i == 1 && dur.is_negative {
                        context.is_p_found = true;
                    } else {
                        return Err("Symbol 'P' occurred at the wrong position".into());
                    }
                }
                'T' => {
                    if context.is_t_found {
                        return Err("Symbol 'T' occurred twice".into());
                    }

                    if context.number > 0 {
                        return Err("Symbol 'T' occurred after a number".into());
                    }

                    context.is_t_found = true;
                    context.last_filled_component = 3;
                }

                // Duration components:
                'Y' => {
                    fill_component(&mut context, &mut dur.years, 1, "years", 'Y')?;
                }
                'M' => {
                    if context.is_t_found {
                        fill_component(&mut context, &mut dur.minutes, 5, "minutes", 'M')?;
                    } else {
                        fill_component(&mut context, &mut dur.months, 2, "months", 'M')?;
                    }
                }
                'D' => {
                    fill_component(&mut context, &mut dur.days, 3, "days", 'D')?;
                }
                'H' => {
                    if !context.is_t_found {
                        return Err("No symbol 'T' found before hours components".into());
                    }
                    fill_component(&mut context, &mut dur.hours, 4, "hours", 'H')?;
                }
                'S' => {
                    if !context.is_t_found {
                        return Err("No symbol 'T' found before seconds components".into());
                    }
                    fill_seconds(&mut context, &mut dur.seconds)?;
                }

                // Number:
                '.' => {
                    if context.is_dot_found {
                        return Err("Dot occurred twice".into());
                    }

                    if context.is_number_empty {
                        return Err("No digits before dot".into());
                    }

                    context.is_dot_found = true;
                }
                digit => {
                    if !digit.is_ascii_digit() {
                        return Err("Incorrect character occurred".into());
                    }

                    if context.is_dot_found {
                        context.numer *= 10;
                        context.numer +=
                            digit.to_digit(10).expect("error converting a digit") as u64;
                        context.denom *= 10;
                    } else {
                        context.number *= 10;
                        context.number +=
                            digit.to_digit(10).expect("error converting a digit") as u64;
                        context.is_number_empty = false;
                    }
                }
            }
        }

        if context.number > 0 {
            return Err("Number at the end of the string".into());
        }

        if !context.is_p_found {
            return Err("'P' must always be present".into());
        }

        if context.last_filled_component == 0 {
            return Err("At least one number and designator are required".into());
        }

        if context.last_filled_component <= 3 && context.is_t_found {
            return Err("No time items are present, so 'T' must not be present".into());
        }

        Ok(dur)
    }
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = if self.is_negative {
            "-P".to_string()
        } else {
            "P".to_string()
        };

        let mut date_str = String::new();
        if self.years > 0 {
            write!(&mut date_str, "{}Y", self.years)?;
        }
        if self.months > 0 {
            write!(&mut date_str, "{}M", self.months)?;
        }
        if self.days > 0 {
            write!(&mut date_str, "{}D", self.days)?;
        }

        let mut time_str = String::new();
        if self.hours > 0 {
            write!(&mut time_str, "{}H", self.hours)?;
        }
        if self.minutes > 0 {
            write!(&mut time_str, "{}M", self.minutes)?;
        }
        if self.seconds > 0.0 {
            write!(&mut time_str, "{}S", self.seconds)?;
        }

        if time_str.is_empty() {
            if date_str.is_empty() {
                s.push_str("0Y");
            } else {
                s.push_str(&date_str);
            }
        } else {
            s.push_str(&date_str);
            s.push('T');
            s.push_str(&time_str);
        }

        write!(f, "{}", s)
    }
}

struct ParsingContext {
    is_p_found: bool,           // Is 'P' found in the string.
    is_t_found: bool,           // Is 'T' delimiter occurred.
    last_filled_component: i32, // 1 to 6 for Year to Minute.

    number: u64,
    is_number_empty: bool,

    is_dot_found: bool,
    // Numerator and denominator of seconds fraction part.
    numer: u64,
    denom: u64,
}

impl ParsingContext {
    pub fn new() -> ParsingContext {
        ParsingContext {
            is_p_found: false,
            is_t_found: false,
            last_filled_component: 0,

            number: 0,
            is_number_empty: true,

            is_dot_found: false,
            numer: 0,
            denom: 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_valid(s: &str) {
        if let Err(err) = Duration::from_str(s) {
            panic!("{} should be valid, but an error occurred: {}", s, err)
        }
    }

    fn check_invalid(s: &str) {
        if Duration::from_str(s).is_ok() {
            panic!("{} should be invalid", s)
        }
    }

    #[test]
    fn duration_parsing_test() {
        check_valid("P2Y6M5DT12H35M30S");
        check_valid("P1DT2H");
        check_valid("P20M");
        check_valid("PT20M");
        check_valid("P0Y20M0D");
        check_valid("P0Y");
        check_valid("-P60D");
        check_valid("PT1M30.5S");

        check_invalid("P-20M");
        check_invalid("P20MT");
        check_invalid("P1YM5D");
        check_invalid("P15.5Y");
        check_invalid("P1D2H");
        check_invalid("1Y2M");
        check_invalid("P2M1Y");
        check_invalid("P");
        check_invalid("PT15.S");
    }

    fn check_std_duration_equal(a: std::time::Duration, b: std::time::Duration) {
        if (a.as_secs_f64() - b.as_secs_f64()).abs() > 1e-4 {
            panic!(
                "assertion `left == right` failed
  left: {:?}
 right: {:?}",
                a, b
            )
        }
    }
    #[test]
    fn convert_duration() {
        // 1.0s
        assert_eq!(
            Duration::from_std_duration(std::time::Duration::new(1, 0)).unwrap(),
            Duration::from_str("PT1S").unwrap()
        );
        check_std_duration_equal(
            Duration::from_str("PT1S")
                .unwrap()
                .to_std_duration()
                .unwrap(),
            std::time::Duration::new(1, 0),
        );

        // 1:40.55
        assert_eq!(
            Duration::from_std_duration(std::time::Duration::new(100, 550_000_000)).unwrap(),
            Duration::from_str("PT1M40.55S").unwrap()
        );
        check_std_duration_equal(
            Duration::from_str("PT1M40.55S")
                .unwrap()
                .to_std_duration()
                .unwrap(),
            std::time::Duration::new(100, 550_000_000),
        );

        // 1:05:30.456
        assert_eq!(
            Duration::from_std_duration(std::time::Duration::new(
                (1 * 60 + 5) * 60 + 30,
                456_000_000
            ))
            .unwrap(),
            Duration::from_str("PT1H5M30.456S").unwrap()
        );
        check_std_duration_equal(
            Duration::from_str("PT1H5M30.456S")
                .unwrap()
                .to_std_duration()
                .unwrap(),
            std::time::Duration::new((1 * 60 + 5) * 60 + 30, 456_000_000),
        );

        // 10day 1:05:30.456
        assert_eq!(
            Duration::from_std_duration(std::time::Duration::new(
                ((10 * 24 + 1) * 60 + 5) * 60 + 30,
                456_000_000
            ))
            .unwrap(),
            Duration::from_str("P10DT1H5M30.456S").unwrap()
        );
        check_std_duration_equal(
            Duration::from_str("P10DT1H5M30.456S")
                .unwrap()
                .to_std_duration()
                .unwrap(),
            std::time::Duration::new(((10 * 24 + 1) * 60 + 5) * 60 + 30, 456_000_000),
        );
    }
}
