
fn main() {
    let s = String::from("hello, ");
    
    let s1 = s; // Modify this line

    s1.push_str("world");

    println!("Success!");
}