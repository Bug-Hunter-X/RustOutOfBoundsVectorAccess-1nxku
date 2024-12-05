fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let second_element = vec[1]; //This line will panic if the index is out of bounds
    println!("The second element is: {}", second_element);
}