fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    match vec.get(1) {
        Some(second_element) => println!("The second element is: {}", second_element),
        None => println!("The vector is empty or does not have a second element."),
    }
} 