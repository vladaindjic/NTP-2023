// Fill the blank to make the code work, do not use 0..2 again
fn main() {
    let s = String::from("hello");

    let slice1 = &s[0..2];

    let slice2 = &s[__];

    assert_eq!(slice1, slice2);

    println!("Success!");
}
