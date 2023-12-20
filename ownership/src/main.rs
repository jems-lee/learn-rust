fn main() {
    let s = String::from("hello");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
    println!("The length of '{s}' is {len}.");

    let mut s = String::from("hello");
    change(&mut s);
    println!("The new string is {s}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
