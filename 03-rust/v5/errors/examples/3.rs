use std::num::ParseIntError;

// With the return type rewritten, we use pattern matching without `unwrap()`.
fn multiply(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    match n1_str.parse::<i32>() {
        Ok(n1)  => {
            match n2_str.parse::<i32>() {
                Ok(n2)  => {
                    Ok(n1 * n2)
                },
                Err(e) => Err(e),
            }
        },
        Err(e) => Err(e),
    }
}

// Rewrite `multiply` to make it less verbose
// You should use `and_then` and `map` here.
fn multiply1(n1_str: &str, n2_str: &str) -> Result<i32, ParseIntError> {
    // IMPLEMENT...
}

fn print(result: Result<i32, ParseIntError>) {
    match result {
        Ok(n)  => println!("n is {}", n),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {

    let twenty = multiply1("10", "2");
    print(twenty);

    let tt = multiply("t", "2");
    print(tt);

    println!("Success!");
}
