use chrono::FixedOffset;

// Parses ISO 8601 timezone.
pub fn parse_timezone(s: &str) -> Result<FixedOffset, String> {
    if s == "Z" {
        return Ok(FixedOffset::east_opt(0).unwrap());
    }

    let tokens: Vec<&str> = s[1..].split(':').collect();
    if tokens.len() != 2 || tokens[0].len() != 2 || tokens[1].len() != 2 {
        return Err("bad timezone format".to_string());
    }
    if !tokens.iter().all(|t| t.chars().all(|c| c.is_ascii_digit())) {
        return Err("bad timezone format".to_string());
    }

    let hours = tokens[0].parse::<i32>().unwrap();
    let minutes = tokens[1].parse::<i32>().unwrap();

    if hours > 14 || (hours == 14 && minutes != 0) || minutes >= 60 {
        return Err("bad timezone format: out of range".to_string());
    }

    let offset_secs = 60 * (60 * hours + minutes);
    match s.chars().next().unwrap() {
        '+' => FixedOffset::east_opt(offset_secs).ok_or("Seconds out of bound".to_owned()),
        '-' => FixedOffset::west_opt(offset_secs).ok_or("Seconds out of bound".to_owned()),
        _ => Err("bad timezone format: timezone should start with '+' or '-'".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timezone_parse_test() {
        // Timezone "Z".
        assert_eq!(parse_timezone("Z"), Ok(FixedOffset::east_opt(0).unwrap()));

        // Positive offset.
        assert_eq!(
            parse_timezone("+06:30"),
            Ok(FixedOffset::east_opt(6 * 3600 + 30 * 60).unwrap())
        );

        // Negative offset.
        assert_eq!(
            parse_timezone("-06:30"),
            Ok(FixedOffset::west_opt(6 * 3600 + 30 * 60).unwrap())
        );

        // Positive offset max.
        assert_eq!(
            parse_timezone("+14:00"),
            Ok(FixedOffset::east_opt(14 * 3600).unwrap())
        );

        // Negative offset max.
        assert_eq!(
            parse_timezone("-14:00"),
            Ok(FixedOffset::west_opt(14 * 3600).unwrap())
        );

        // Invalid values.
        assert!(parse_timezone("06:30").is_err());
        assert!(parse_timezone("-06").is_err());
        assert!(parse_timezone("-06:30:00").is_err());
        assert!(parse_timezone("-06:000").is_err());
        assert!(parse_timezone("-06:0").is_err());
        assert!(parse_timezone("-006:00").is_err());
        assert!(parse_timezone("-6:00").is_err());
        assert!(parse_timezone("-06:60").is_err());
        assert!(parse_timezone("-15:00").is_err());
        assert!(parse_timezone("-14:01").is_err());
        assert!(parse_timezone("+14:01").is_err());
        assert!(parse_timezone("---:--").is_err());
        assert!(parse_timezone("+--:--").is_err());
        assert!(parse_timezone("-AA:AA").is_err());
        assert!(parse_timezone("-Z").is_err());
        assert!(parse_timezone("+Z").is_err());
        assert!(parse_timezone("ZZ").is_err());
    }
}
