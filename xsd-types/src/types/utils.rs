use chrono::FixedOffset;

// Parses ISO 8601 timezone.
pub fn parse_timezone(s: &str) -> Result<FixedOffset, String> {
    if s == "Z" {
        return Ok(FixedOffset::east(0));
    }

    let tokens: Vec<&str> = s[1..].split(":").collect();
    if tokens.len() != 2 || tokens[0].len() != 2 || tokens[1].len() != 2 {
        return Err("bad timezone format".to_string());
    }

    let hours = tokens[0].parse::<i32>().unwrap();
    let minutes = tokens[1].parse::<i32>().unwrap();

    if hours > 14 || (hours == 14 && minutes != 0) || minutes >= 60 {
        return Err("bad timezone format: out of range".to_string());
    }

    let offset_secs = 60 * (60 * hours + minutes);
    match s.chars().next().unwrap() {
        '+' => Ok(FixedOffset::east(offset_secs)),
        '-' => Ok(FixedOffset::west(offset_secs)),
        _ => Err("bad timezone format: timezone should start with '+' or '-'".to_string()),
    }
}
