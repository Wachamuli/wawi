pub fn seconds_to_hour_minute(seconds: i64) -> String {
    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;

    match (hours, minutes) {
        (h, _) if h > 1 => format!("{h} hours"),
        (_, m) if m > 1 => format!("{m} minutes"),
        (1, _) => format!("1 hour"),
        (_, 1) => format!("1 minute"),
        _ => String::from("Less than a minute"),
    }
}

pub fn kebab_to_title_case(string: &String) -> String {
    string
        .split("-")
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(n) => n.to_uppercase().to_string() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}
