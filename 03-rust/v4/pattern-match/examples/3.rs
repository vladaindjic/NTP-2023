
fn main() {
    let o = Some(7);

    // Remove the whole `match` block, use `if let` instead 
    match o {
        Some(i) => {
            println!("This is a really long string and `{:?}`", i);

            println!("Success!");
        }
        _ => {}
    };
}