pub fn raindrops(number: i32) -> String {
    let mut output = String::new();
    if number % 3 == 0 { output.push_str("Pling"); }
    if number % 5 == 0 { output.push_str("Plang"); }
    if number % 7 == 0 { output.push_str("Plong"); }
    if output.is_empty() { output.push_str(number.to_string().as_str()); }
    output
}
