# V3 - Programski jezik Rust

## Vlasništvo i pozajmljivanje

1. What is ownership in Rust?
2. Name three ownership rules?
3. What is a scope?
4. Is a string literal mutable or immutable?
5. Are variables of type `String` mutable or immutable?
6. When is the following String deallocated?

```rust
{
    let s = String::from("hello"); // s is valid from this point forward

    // do stuff with s
}      
```

7. What is builtin Rust function `drop` used for?
8. What is the internal organization of a `String` object? What is kept on the stack and what on the heap?
9. What is the difference between a shallow-copy and a deep-copy?
10. If we assign `let s2 = s1`, does Rust perform a shallow-copy, a deep-copy, or something else?
11. What is the concept of moving variables (ownership)?
12. What is the output of the following snippet?

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1);
  }
```

13. What does `Copy` trait provide?
14. What types can implement the `Copy` trait? Name a few examples.
15. Does a reference have the ownership of an object it points to?
16. What is the concept of borrowing?
17. Are references mutable by default?
18. Can a reference be mutable?
19. How many mutable reference to a single object can exist within the same scope? Why?
20. Under which three conditions do data races occur?
21. Can both mutable and immutable reference exists within the same code at the same time?
22. Is the following code snippet valid?

```rust
fn main() {
  let mut s = String::from("hello");

  let r1 = &s; // no problem
  let r2 = &s; // no problem
  println!("{} and {}", r1, r2);
  // variables r1 and r2 will not be used after this point

  let r3 = &mut s; // no problem
  println!("{}", r3);
}
```

23. What is the scope (lifetime) of a reference?
24. What is the dangling pointer (reference)?
25. Fix the error in the following snippet.

```rust
fn main() {
  let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

26. Is the following statement true? At any given time, you can have either one mutable reference or any number of immutable references
27. What is a slice type?
28. How do we specify a string slice?
29. What is the output of the following snippet?

```rust
fn first_word(s: &String) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();

    println!("the first word is: {}", word);
}
```

30. Are string literals string slices? If they are, are they mutable or immutable?
31. What is the type of a slice in the following snippet?

```rust
let a = [1, 2, 3, 4, 5];

let slice = &a[1..3];
```

32. Does the following snippet compile successfully? Why?

```rust
fn first_word(s: &str) -> &str {
  let bytes = s.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
      if item == b' ' {
          return &s[0..i];
      }
  }

  &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
}
```

33. Fix the errors in the following code:
    ```rust
    fn main() {
      let s = String::from("hello");  // s comes into scope

      takes_ownership(s);             
      println!("{}", s);

      let x = 5;                      

      makes_copy(x);   
      
      println!("{}", x)
    } 

    fn takes_ownership(some_string: String) {
        println!("{}", some_string);
    } 
    fn makes_copy(some_integer: i32) {
        println!("{}", some_integer);
    }
    ```
34. Does the following snippet produce a compile-time error?
    ```rust
    fn main() {
      let s1 = gives_ownership(); 

      let s2 = String::from("hello");

      let s3 = takes_and_gives_back(s2);
    }

    fn gives_ownership() -> String {
        let some_string = String::from("yours");

        some_string 
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }
    ```