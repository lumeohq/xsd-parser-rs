use std::time;

#[derive(Default, Debug)]
pub struct Duration {
    pub negative: bool,

    pub years: u64,
    pub months: u64,
    pub days: u64,

    pub hours: u64,
    pub minutes: u64,
    pub seconds: f64,
}

impl Duration {
    pub fn to_lexical_representation(&self) -> String {
        let mut s = if self.negative {"-P".to_string()} else {"P".to_string()};

        let mut date_str = String::new();
        if self.years > 0 {
            date_str.push_str(&format!("{}Y", self.years));
        }
        if self.months > 0 {
            date_str.push_str(&format!("{}M", self.months));
        }
        if self.days > 0 {
            date_str.push_str(&format!("{}D", self.days));
        }

        let mut time_str = String::new();
        if self.hours > 0 {
            date_str.push_str(&format!("{}H", self.hours));
        }
        if self.minutes > 0 {
            date_str.push_str(&format!("{}M", self.minutes));
        }
        if self.seconds > 0.0 {
            date_str.push_str(&format!("{}S", self.seconds));
        }

        if time_str.is_empty() {
            if date_str.is_empty() {
                s.push_str("0Y");
            }
            else {
                s.push_str(&date_str);
            }
        }
        else {
            s.push_str(&date_str);
            s.push_str("T");
            s.push_str(&time_str);
        }

        s
    }

    pub fn to_std_duration(&self) -> Result<std::time::Duration, &'static str> {
        if self.years > 0 || self.months > 0 {
            Err("Duration with months or years require a starting date to be converted")
        }
        else {
            let secs = self.seconds as u64;

            let nanos = ((self.seconds - secs as f64) * 1e9) as u32;
            let secs = secs + 60 * (self.minutes + 60 * (self.hours + 24 * self.days));

            Ok(std::time::Duration::new(secs, nanos))
        }
    }

    // TODO: Implement normalization function that takes a moment at time to start from and
    // converts months & years to days.

    // TODO: Add a version of to_std_duration that takes a moment at time to start from and uses
    // normalization function.

    // TODO: rewrite to reduce code duplication.
    pub fn from_lexical_representation(s: &str) -> Result<Duration, &'static str> {
        let mut dur: Duration = Default::default();

        let mut p_found = false;
        let mut t_found = false;
        let mut last_component = 0;

        let mut cur: u64 = 0;
        let mut cur_started = false;

        let mut dot_found = false;
        let mut numer: u64 = 0;
        let mut denom: u64 = 1;

        for (i, c) in s.chars().enumerate() {
            if c == '-' {
                if i == 0 {
                    dur.negative = true;
                }
                else {
                    return Err("'-' sign may only occur at the beginning of the string");
                }
            }
            else if c == 'P' {
                if i == 0 || i == 1 && dur.negative {
                    p_found = true;
                }
                else {
                    return Err("Symbol 'P' occurred at the wrong position");
                }
            }
            else if c == 'T' {
                if t_found {
                    return Err("Symbol 'T' occurred twice");
                }

                if cur > 0 {
                    return Err("Symbol 'T' occurred after a number");
                }

                t_found = true;
                last_component = 3;
            }
            else if c == '.' {
                if dot_found {
                    return Err("Dot occurred twice");
                }

                if !cur_started {
                    return Err("No digits before dot");
                }

                dot_found = true;
            }
            else if c.is_digit(10) {
                if dot_found {
                    numer *= 10;
                    numer += c.to_digit(10).expect("error converting a digit") as u64;
                    denom *= 10;
                }
                else {
                    cur *= 10;
                    cur += c.to_digit(10).expect("error converting a digit") as u64;
                    cur_started = true;
                }
            }
            else if c == 'Y' {
                if !cur_started {
                    return Err("No value is specified for years, so 'Y' must not be present");
                }

                if dot_found {
                    return Err("Only the seconds can be expressed as a decimal");
                }

                if last_component >= 1 {
                    return Err("Bad order of duration components");
                }

                last_component = 1;
                dur.years = cur;
                cur = 0;
                cur_started = false;
            }
            else if c == 'M' {
                if t_found {
                    if !cur_started {
                        return Err("No value is specified for minutes, so 'M' must not be present");
                    }

                    if dot_found {
                        return Err("Only the seconds can be expressed as a decimal");
                    }

                    if last_component >= 5 {
                        return Err("Bad order of duration components");
                    }

                    last_component = 5;
                    dur.minutes = cur;
                    cur = 0;
                    cur_started = false;
                }
                else {
                    if !cur_started {
                        return Err("No value is specified for months, so 'M' must not be present");
                    }

                    if dot_found {
                        return Err("Only the seconds can be expressed as a decimal");
                    }

                    if last_component >= 2 {
                        return Err("Bad order of duration components");
                    }

                    last_component = 2;
                    dur.months = cur;
                    cur = 0;
                    cur_started = false;
                }
            }
            else if c == 'D' {
                if !cur_started {
                    return Err("No value is specified for days, so 'D' must not be present");
                }

                if dot_found {
                    return Err("Only the seconds can be expressed as a decimal");
                }

                if last_component >= 3 {
                    return Err("Bad order of duration components");
                }

                last_component = 3;
                dur.days = cur;
                cur = 0;
                cur_started = false;
            }
            else if c == 'H' {
                if !cur_started {
                    return Err("No value is specified for hours, so 'H' must not be present");
                }

                if dot_found {
                    return Err("Only the seconds can be expressed as a decimal");
                }

                if !t_found {
                    return Err("No symbol 'T' found before hours components");
                }

                if last_component >= 4 {
                    return Err("Bad order of duration components");
                }

                last_component = 4;
                dur.hours = cur;
                cur = 0;
                cur_started = false;
            }
            else if c == 'S' {
                if !cur_started {
                    return Err("No value is specified for seconds, so 'S' must not be present");
                }

                if dot_found && denom == 1 {
                    return Err("At least one digit should occur after dot");
                }

                if !t_found {
                    return Err("No symbol 'T' found before seconds components");
                }

                if last_component >= 6 {
                    return Err("Bad order of duration components");
                }

                last_component = 6;
                dur.seconds = cur as f64 + numer as f64 / denom as f64;
                cur = 0;
                cur_started = false;
            }
            else {
                return Err("Incorrect character occured");
            }
        }

        if cur > 0 {
            return Err("Number at the end of the string");
        }

        if !p_found {
            return Err("Symbol 'P' not found in the string");
        }

        if last_component == 0 {
            return Err("No duration components presented");
        }

        if last_component <= 3 && t_found {
            return Err("no time items are present, so 'T' must not be present");
        }

        Ok(dur)
    }
}
