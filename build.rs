fn main() {
    let families: Vec<_> = std::env::vars().filter_map(|(key, _value)| {
        if key.starts_with("CARGO_FEATURE_STM32") {
            Some(key[14..].to_ascii_lowercase())  // Strip 'CARGO_FEATURE_'
        } else {
            None
        }
    }).collect();

    if families.is_empty() {
        println!("No family features selected");
        std::process::exit(1);
    }
    if families.len() > 1 {
        println!("More than one family feature selected: {:?}", families);
        std::process::exit(1);
    }
}
