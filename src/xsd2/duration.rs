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
            time_str.push_str(&format!("{}H", self.hours));
        }
        if self.minutes > 0 {
            time_str.push_str(&format!("{}M", self.minutes));
        }
        if self.seconds > 0.0 {
            time_str.push_str(&format!("{}S", self.seconds));
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

    // TODO: Add a version of to_std_duration that takes a moment at time to start from.

    // TODO: Implement normalization function that takes a moment at time to start from and
    // converts months & years to days.

    pub fn from_lexical_representation(s: &str) -> Result<Duration, &'static str> {
        fn fill_component(context: &mut ParsingContext, component: &mut u64, idx: i32, name: &str, symbol: char) -> Option<&'static str> {
            if context.number_is_empty {
                return Some("No value is specified for years, so 'Y' must not be present");
            }

            if context.dot_found {
                return Some("Only the seconds can be expressed as a decimal");
            }

            if context.last_filled_component >= idx {
                return Some("Bad order of duration components");
            }

            *component = context.number;
            context.last_filled_component = idx;
            context.number = 0;
            context.number_is_empty = true;
            None
        }

        fn fill_seconds(context: &mut ParsingContext, seconds: &mut f64) -> Option<&'static str> {
            if context.number_is_empty {
                return Some("No value is specified for seconds, so 'S' must not be present");
            }

            if context.dot_found && context.denom == 1 {
                return Some("At least one digit must follow the decimal point if it appears");
            }

            if context.last_filled_component >= 6 {
                return Some("Bad order of duration components");
            }

            *seconds = context.number as f64 + context.numer as f64 / context.denom as f64;
            context.last_filled_component = 6;
            context.number = 0;
            context.number_is_empty = true;
            None
        }

        let mut dur: Duration = Default::default();
        let mut context = ParsingContext::new();
        for (i, c) in s.chars().enumerate() {
            match c {
                '-' => {
                    if i == 0 {
                        dur.negative = true;
                    }
                    else {
                        return Err("The minus sign must appear first");
                    }
                }
                'P' => {
                    if i == 0 || i == 1 && dur.negative {
                        context.p_found = true;
                    }
                    else {
                        return Err("Symbol 'P' occurred at the wrong position");
                    }
                }
                'T' => {
                    if context.t_found {
                        return Err("Symbol 'T' occurred twice");
                    }

                    if context.number > 0 {
                        return Err("Symbol 'T' occurred after a number");
                    }

                    context.t_found = true;
                    context.last_filled_component = 3;
                }

                // Duration components:
                'Y' => if let Some(e) = fill_component(&mut context, &mut dur.years, 1, "years", 'Y') {
                        return Err(e);
                }
                'M' => {
                    if context.t_found {
                        if let Some(e) = fill_component(&mut context, &mut dur.minutes, 5, "minutes", 'M') {
                            return Err(e);
                        }
                    }
                    else {
                        if let Some(e) = fill_component(&mut context, &mut dur.months, 2, "months", 'M') {
                            return Err(e);
                        }
                    }
                }
                'D' => if let Some(e) = fill_component(&mut context, &mut dur.days, 3, "days", 'D') {
                        return Err(e);
                }
                'H' => {
                    if !context.t_found { return Err("No symbol 'T' found before hours components") }
                    if let Some(e) = fill_component(&mut context, &mut dur.hours, 4, "hours", 'H') {
                        return Err(e);
                    }
                }
                'S' => {
                    if !context.t_found { return Err("No symbol 'T' found before seconds components") }
                    if let Some(e) = fill_seconds(&mut context, &mut dur.seconds) {
                        return Err(e);
                    }
                }

                // Number:
                '.' => {
                    if context.dot_found {
                        return Err("Dot occurred twice");
                    }

                    if context.number_is_empty {
                        return Err("No digits before dot");
                    }

                    context.dot_found = true;
                }
                digit => {
                    if !digit.is_digit(10) {
                        return Err("Incorrect character occurred");
                    }

                    if context.dot_found {
                        context.numer *= 10;
                        context.numer += digit.to_digit(10).expect("error converting a digit") as u64;
                        context.denom *= 10;
                    }
                    else {
                        context.number *= 10;
                        context.number += digit.to_digit(10).expect("error converting a digit") as u64;
                        context.number_is_empty = false;
                    }
                }
            }
        }

        if context.number > 0 {
            return Err("Number at the end of the string");
        }

        if !context.p_found {
            return Err("'P' must always be present");
        }

        if context.last_filled_component == 0 {
            return Err("At least one number and designator are required");
        }

        if context.last_filled_component <= 3 && context.t_found {
            return Err("no time items are present, so 'T' must not be present");
        }

        Ok(dur)
    }
}

struct ParsingContext {
    p_found: bool, // Is 'P' found in the string.
    t_found: bool, // Is 'T' delimiter occurred.
    last_filled_component: i32,  // 1 to 6 for Year to Minute.

    number: u64,
    number_is_empty: bool,

    dot_found: bool,
    // Numerator and denominator of seconds fraction part.
    numer: u64,
    denom: u64,
}

impl ParsingContext {
    pub fn new() -> ParsingContext {
        ParsingContext {
            p_found: false,
            t_found: false,
            last_filled_component: 0,

            number: 0,
            number_is_empty: true,

            dot_found: false,
            numer: 0,
            denom: 1,
        }
    }
}