# V2 - Programski jezik Rust

## Osnovni programski koncepti

1. Are variables mutable by default?
2. Fix compile error in the following code:

```rust 
fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
```

3. What are the main differences between constants and immutable variables?
4. What is the lifetime of a constant?
5. Determine the output of the following code:

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

6. What is the difference between variable shadowing and mutability?
7. Is Rust a statically or dynamically typed language?
8. Does the following snippet produce an error? If so, fix it:

```rust
let guess = "42".parse().expect("Not a number!");
```

9. What is the size of `isize` and `usize`?
10. Is `57u8` a valid literal?
11. Advanced: How does Rust handle [Integer Overflow](https://en.wikipedia.org/wiki/Integer_overflow)?
12. What is the size of a `bool`? How about `char`?
13. What are primitive compound types?
14. Where are arrays allocated in memory?
15. What are the elements of `let a = [3; 5];`? What do the `3` and `5` mean?
16. What is the entry point in the Rust program?
17. Explain the term that Rust is expression-based language?
18. Is a scope an expression? Support the answer with an appropriate example
19. What does `;` serve for?
20. Is the following function definition valid?

```rust
fn five() -> i32 {
  5
}
```

21. What is the output of the program shown in stdout?

```rust
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

22. What are the line comments in Rust?
23. What are "arms" in an expression in Rust?
24. What is the output of the following code?

```rust
fn main() {
    let number = 3;

    if number {
        println!("number was three");
    }
}
```

25. Is an `if` statement an expression in Rust?
26. What is the output of the following code?

```rust
fn main() {
    let condition = true;

    let number = if condition { 5 } else { "six" };

    println!("The value of number is: {number}");
}
```

27. What is the output of the following snippet?

```rust
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
```

28. What do each of the break statements breaks into in the following example?

```rust
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
```

Additional excercises: Check excercises from the end of [chapter](https://doc.rust-lang.org/book/ch03-05-control-flow.html).
