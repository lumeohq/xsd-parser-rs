use std::time;

// TODO: decimal seconds

#[derive(Default)]
pub struct Duration {
    pub negative: bool,

    pub years: u64,
    pub months: u64,
    pub days: u64,

    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
}

impl Duration {
    pub fn to_lexical_representation(&self) -> String {
        let mut s = if self.negative {"-P".to_string()} else {"P".to_string()};

        let mut date_str = String::new();
        if self.years > 0 {
            date_str.push_str(&self.years.to_string());
            date_str.push_str("Y");
        }
        if self.months > 0 {
            date_str.push_str(&self.months.to_string());
            date_str.push_str("M");
        }
        if self.days > 0 {
            date_str.push_str(&self.days.to_string());
            date_str.push_str("D");
        }

        let mut time_str = String::new();
        if self.hours > 0 {
            time_str.push_str(&self.hours.to_string());
            time_str.push_str("H");
        }
        if self.minutes > 0 {
            time_str.push_str(&self.minutes.to_string());
            time_str.push_str("M");
        }
        if self.seconds > 0 {
            time_str.push_str(&self.seconds.to_string());
            time_str.push_str("S");
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
            Ok(std::time::Duration::from_secs(
                self.seconds + 60 * (self.minutes + 60 * (self.hours + 24 * self.days))))
        }
    }

    pub fn from_lexical_representation(s: &str) -> Result<Duration, &'static str> {
        let mut dur: Duration = Default::default();

        let mut cur: u64 = 0;
        let mut cur_started = false;
        let mut p_found = false;
        let mut t_found = false;
        let mut last_component = 0;
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
                    return Err("Symbol 'T' occurred twice")
                }

                if cur > 0 {
                    return Err("Symbol 'T' occurred after a number")
                }

                t_found = true;
                last_component = 3;
            }
            else if c.is_digit(10) {
                cur *= 10;
                cur += c.to_digit(10).expect("error converting a digit") as u64;
                cur_started = true;
            }
            else if c == 'Y' {
                if (!cur_started) {
                    return Err("No value is specified for years, so 'Y' must not be present")
                }

                if last_component >= 1 {
                    return Err("Bad order of duration components")
                }

                last_component = 1;
                dur.years = cur;
                cur = 0;
                cur_started = false;
            }
            else if c == 'M' {
                if t_found {
                    if (!cur_started) {
                        return Err("No value is specified for minutes, so 'M' must not be present")
                    }

                    if last_component >= 5 {
                        return Err("Bad order of duration components")
                    }

                    last_component = 5;
                    dur.minutes = cur;
                    cur = 0;
                    cur_started = false;
                }
                else {
                    if (!cur_started) {
                        return Err("No value is specified for months, so 'M' must not be present")
                    }

                    if last_component >= 2 {
                        return Err("Bad order of duration components")
                    }

                    last_component = 2;
                    dur.months = cur;
                    cur = 0;
                    cur_started = false;
                }
            }
            else if c == 'D' {
                if (!cur_started) {
                    return Err("No value is specified for days, so 'D' must not be present")
                }

                if last_component >= 3 {
                    return Err("Bad order of duration components")
                }

                last_component = 3;
                dur.days = cur;
                cur = 0;
                cur_started = false;
            }
            else if c == 'H' {
                if (!cur_started) {
                    return Err("No value is specified for hours, so 'H' must not be present")
                }

                if !t_found {
                    return Err("No symbol 'T' found before hours components")
                }

                if last_component >= 4 {
                    return Err("Bad order of duration components")
                }

                last_component = 4;
                dur.hours = cur;
                cur = 0;
                cur_started = false;
            }
            else if c == 'S' {
                if (!cur_started) {
                    return Err("No value is specified for seconds, so 'S' must not be present")
                }

                if !t_found {
                    return Err("No symbol 'T' found before seconds components")
                }

                if last_component >= 6 {
                    return Err("Bad order of duration components")
                }

                last_component = 6;
                dur.hours = cur;
                cur = 0;
                cur_started = false;
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
