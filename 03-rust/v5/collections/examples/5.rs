#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}
fn main() {
    // Fill in the blank
    let v : Vec<IpAddr>= __;
    
    // Comparing two enums need to derive the PartialEq trait
    assert_eq!(v[0], IpAddr::V4("127.0.0.1".to_string()));
    assert_eq!(v[1], IpAddr::V6("::1".to_string()));

    println!("Success!");
}
