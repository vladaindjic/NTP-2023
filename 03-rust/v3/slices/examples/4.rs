
// Fix errors
fn main() {
    let mut s = String::from("hello world");

    // Here, &s is `&String` type, but `first_word` need a `&str` type.
    // This works because `&String` can be implicitly converted to `&str. This is called `Deref coercion`. 
    let word = first_word(&s);

    s.clear(); // error!

    println!("the first word is: {}", word);
}
fn first_word(s: &str) -> &str {
    &s[..1]
}