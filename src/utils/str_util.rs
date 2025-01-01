use std::string::String;

pub fn capitalize(input: &String) -> String {
    let mut chars = input.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

pub fn map_to(vec: Vec<&str>) -> Vec<String> {
    vec.iter().map(|&s| s.to_string()).collect()
}
