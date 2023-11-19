use clap::{App, Arg};
use std::str::FromStr;

fn parse_time(time_str: &str) -> Result<(u32, u32, bool), String> {
    // Split the string into time and period (AM/PM) parts
    let (time_part, period_part) = if time_str.ends_with("AM") || time_str.ends_with("PM") {
        let len = time_str.len();
        (&time_str[..len - 2], &time_str[len - 2..])
    } else {
        return Err("Invalid time format".to_string());
    };

    let time_parts: Vec<&str> = time_part.split(':').collect();
    if time_parts.len() != 2 {
        return Err("Invalid time format".to_string());
    }

    let hours = u32::from_str(time_parts[0].trim())
        .map_err(|_| format!("Invalid hour {}", time_parts[0]))?;
    let minutes = u32::from_str(time_parts[1].trim())
        .map_err(|_| format!("Invalid minute {}", time_parts[1]))?;

    if hours == 0 || hours > 12 {
        return Err(format!("Invalid hour {}", hours).to_string());
    }

    if minutes >= 60 {
        return Err(format!("Invalid minute {}", minutes).to_string());
    }

    let is_pm = match period_part {
        "AM" => false,
        "PM" => true,
        _ => return Err("Invalid AM/PM format".to_string()),
    };

    Ok((hours, minutes, is_pm))
}

fn calculate_duration(start_time: &str, end_time: &str) -> Result<String, String> {
    let (start_hours, start_minutes, start_is_pm) = parse_time(start_time)?;
    let (end_hours, end_minutes, end_is_pm) = parse_time(end_time)?;

    let start_total_minutes = if start_hours == 12 {
        start_minutes
    } else {
        start_hours * 60 + start_minutes
    } + if start_is_pm { 12 * 60 } else { 0 };

    let end_total_minutes = if end_hours == 12 {
        end_minutes
    } else {
        end_hours * 60 + end_minutes
    } + if end_is_pm { 12 * 60 } else { 0 };

    let mut duration = if end_total_minutes >= start_total_minutes {
        end_total_minutes - start_total_minutes
    } else {
        24 * 60 - (start_total_minutes - end_total_minutes)
    };

    let hours = duration / 60;
    duration %= 60;
    let minutes = duration;

    Ok(format!("{} hours {} minutes", hours, minutes))
}

fn main() {
    let matches = App::new("Duration Calculator")
        .version("1.0")
        .about("Calculates duration between two times")
        .arg(
            Arg::with_name("start_time")
                .help("Start time in the format HH:MMAM/PM")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("end_time")
                .help("End time in the format HH:MMAM/PM")
                .required(true)
                .index(2),
        )
        .get_matches();

    let start_time = matches.value_of("start_time").unwrap();
    let end_time = matches.value_of("end_time").unwrap();

    match calculate_duration(start_time, end_time) {
        Ok(duration) => println!("Duration: {}", duration),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_time_with_space() {
        assert_eq!(parse_time("12:30 AM"), Ok((12, 30, false)));
        assert_eq!(parse_time("3:45 PM"), Ok((3, 45, true)));
    }

    #[test]
    fn test_parse_time_without_space() {
        assert_eq!(parse_time("12:30AM"), Ok((12, 30, false)));
        assert_eq!(parse_time("3:45PM"), Ok((3, 45, true)));
    }

    #[test]
    fn test_parse_time_invalid_format() {
        assert!(parse_time("1230AM").is_err());
        assert!(parse_time("3:45").is_err());
    }

    #[test]
    fn test_calculate_duration_am_to_pm() {
        assert_eq!(
            calculate_duration("12:44 AM", "3:45 PM"),
            Ok("15 hours 1 minutes".to_string())
        );
    }

    #[test]
    fn test_calculate_duration_pm_to_am() {
        assert_eq!(
            calculate_duration("8:30 PM", "5:15 AM"),
            Ok("8 hours 45 minutes".to_string())
        );
    }

    #[test]
    fn test_calculate_duration_within_am() {
        assert_eq!(
            calculate_duration("2:00 AM", "4:30 AM"),
            Ok("2 hours 30 minutes".to_string())
        );
    }

    #[test]
    fn test_calculate_duration_within_pm() {
        assert_eq!(
            calculate_duration("1:15 PM", "4:45 PM"),
            Ok("3 hours 30 minutes".to_string())
        );
    }

    #[test]
    fn test_calculate_duration_invalid_input() {
        assert!(calculate_duration("12:600 AM", "3:45 PM").is_err());
        assert!(calculate_duration("12:30 AM", "3a:60 PM").is_err());
    }
}
