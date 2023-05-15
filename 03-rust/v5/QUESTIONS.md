# V5 - Programski jezik Rust

## Paketi, moduli, kolekcije, obrada grešaka, testiranje

1. What are the elements of the Rust's module system?
2. What is a crate?
3. What is a binary crate?
4. What is a library crate?
5. What is the crate root?
6. What is a package?
7. How many binary or library crates can a package contain?
8. What is the crate root of a binary crate by the default? What about the library crates?
9. Where else can binary crates reside?
10. What conventions should you follow when defining a modules within crates?
11. Are modules private by the default?
12. How do we form a path and refer to an item (method or function) in the module tree?
13. Does a parent module have access to content of an inner module by default?
14. Call the function `add_to_waitlist` by using both absolute and relative paths at the TODO placeholders:

        ```rust
        mod front_of_house {
        mod hosting {
            fn add_to_waitlist() {}
        }
        }

        pub fn eat_at_restaurant() {
            // TODO: Absolute path
            
            // TODO: Relative path
        }
        ```

15. Considering the answer of the previous question, why can `each_at_restaurant` access the private module `front_of_house`?
16. What is `super` used for?
17. Fix the error in the following code:

        ```rust
        mod back_of_house {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }
        }

        pub fn eat_at_restaurant() {
            // Order a breakfast in the summer with Rye toast
            let mut meal = back_of_house::Breakfast::summer("Rye");
            // Change our mind about what bread we'd like
            meal.toast = String::from("Wheat");
            println!("I'd like {} toast please", meal.toast);

            meal.seasonal_fruit = String::from("blueberries");
        }

        fn main() {
            eat_at_restaurant();
        }
        ```

18. Why do we need function `summer` to be `public`?
19. If an enum is public, are all of its variants public too? What about structs?
20. Does the following code snippet produce an error? Why?

        ```rust
        mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
        }

        use crate::front_of_house::hosting;

        mod customer {
            pub fn eat_at_restaurant() {
                hosting::add_to_waitlist();
            }
        } 
        ```

21. Name the two ways to fix the following error produced by the snippet:

        ```rust
        mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {}
        }
        }

        use crate::front_of_house::hosting;

        mod customer {
            pub fn eat_at_restaurant() {
                hosting::add_to_waitlist();
            }
        }

        fn main() {
            customer::eat_at_restaurant();
        ```

22. What is the process of re-exporting names?
23. How do we add and use an external package as a dependency in our Rust project?
24. What is the name of the standard library crate?
25. Rewrite more concisely the following snippet:

        ```rust
        use std::cmp::Ordering;
        use std::io;
        ```

26. Rewrite more concisely the following snippet:

        ```rust
        use std::io;
        use std::io::Write;
        ```

27. Where does the Rust store builtin arrays and tuples?
28. Where are elements of the collections like vector and map stored?
29. Fix the error in the following snippet:

        ```rust
        fn main() {
        let v = Vec::new();
        }

        ```

30. Fix the following code snippet:

        ```rust
        fn main() {
        let v = Vec::new();

        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        }
        ```

31. What is the output of the following program? Why? Provide an alternative if the program produces an error.

        ```rust
        fn main() {
                let v = vec![1, 2, 3, 4, 5];
                
                let does_not_exist = &v[100];
        }
        ```

32. Does the following program produce an error? Why?

        ```rust
        fn main() {
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];

        v.push(6);

        println!("The first element is: {first}");
        }
        ```

33. Do the borowing rules apply on vectors (given the previous example)? Why?
34. Fix the error in the following snippet:

        ```rust
        fn main() {
        let mut v = vec![100, 32, 57];
        for i in &v {
            i += 50;
        }
        }
        ```

35. Does the following code snippet raise an error? Why?

        ```rust
        n main() {
        let mut v = vec![100, 32, 57];
        for i in &mut v {
            *i += 50;
            v.push(55)
        }
        }
        ```

36. Is the following code snippet valid?

        ```rust
        fn main() {
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];
        }
        ```

37. When are the vector elements dropped?
38. What are Strings and what are string slices (str or &str)?
39. Rewrite the following snippet by using `to_string` method on literals:

        ```rust
        fn main() {
        let s = String::from("initial contents");
        }
        ```

40. Does `push_str` takes ownership of the parameter? Check the example below:

        ```rust
        fn main() {
        let mut s1 = String::from("foo");
        let s2 = "bar";
        s1.push_str(s2);
        println!("s2 is {s2}");
        }
        ```

41. Fix an error in the following snippet:

        ```rust
        fn main() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + s2; // note s1 has been moved here and can no longer be used
        }
        ```

42. Does the following snippet produce an error? Why?

        ```rust
        fn main() {
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
        println!("{s1}, {s2}")
        }
        ```

43. What is the output of the following code?

        ```rust
        fn main() {
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let s = format!("{s1}-{s2}-{s3}");
        // let s = s1 + "-" + &s2 + "-" + &s3;   Solution: Please, uncomment me!
        println!("{s1}, {s2}, {s3}: {s}")
        }
        ```

44. Does the following code produce an error? Why?

        ```rust
        fn main() {
        let s1 = String::from("hello");
        let h = s1[0];
        }
        ```

45. What is the output of the following snippet?

        ```rust
        #![allow(unused)]
        fn main() {
            let hello = "Здравo";
            let s = &hello[0..4];
            println!("{s}")
        }
        ```

46. Does the following snippet panic?

        ```rust
        #![allow(unused)]
        fn main() {
            let hello = "Здравo";
            let s = &hello[0..1];
            println!("{s}")
        }
        ```

47. Do the outputs of the following two snippets differ?

        ```rust
        // Snippet 1
        #![allow(unused)]
        fn main() {
            for c in "Зд".chars() {
                println!("{c}");
            }
        }
        // Snippet 2
        #![allow(unused)]
        fn main() {
            for b in "Зд".bytes() {
                println!("{b}");
            }
        }
        ```

48. What are `copied` and `unwrap_or` used for in the following snippet?

        ```rust
        fn main() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        let team_name = String::from("Blue");
        let score = scores.get(&team_name).copied().unwrap_or(0);
        }  
        ```

49. Iterate over keys and values of the score hash map given this code snippet:

        ```rust
        fn main() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);

        // TODO: Add code here
        }
        ```

50. Find the errors in the following snippet:

        ```rust
        fn main() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        let blue = String::from("Blue");
        let ten = 10;
        let yellow = String::from("Yellow");
        let fifty = 50;

        scores.insert(blue, ten);
        scores.insert(yellow, fifty);

        println!("{blue}, {ten}; {yellow}, {fifty}")
        }
        ```

51. What is the output of the following code?

        ```rust
        fn main() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Blue"), 25);

        println!("{:?}", scores);
        }
        ```

52. What does the following snippet print to the stdout?

        ```rust
        fn main() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        scores.insert(String::from("Blue"), 10);

        scores.entry(String::from("Yellow")).or_insert(50);
        scores.entry(String::from("Blue")).or_insert(50);

        println!("{:?}", scores);
        }
        ```

53. How does the `or_entry` method on `Entry` work?
54. What is the output shown in the stdout after executing the following snippet?

        ```rust
        fn main() {
        use std::collections::HashMap;

        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }

        println!("{:?}", map);
        }
        ```

55. What types of errors does Rust support and how are they handled?
56. How can we cause a panic in a program?
57. What is unwinding and how to prevent it?
58. Does the following program panic? If so, how can we show the complete backtrace?

        ```rust
        fn main() {
        let v = vec![1, 2, 3];

        v[99];
        }
        ```

59. Try to open file `hello.txt` if the file exists, otherwise panic. Use `match` to solve the problem.
60. After studying closures, try to solve the previous assignment by using lexical closures.
61. Try to open file `hello.txt` if the file exists, otherwise panic. Use `unwrap` method to solve the problem.
62. Solve the previous assignment by using `expect`.
63. Which is more preferable to use: `unwrap` or `expect`?
64. Write shorter code equivalent to the following snippet by using `?`. Using `fs::read_to_string` is not allowed:

        ```rust
        #![allow(unused)]
        fn main() {
        use std::fs::File;
        use std::io::{self, Read};

        fn read_username_from_file() -> Result<String, io::Error> {
            let username_file_result = File::open("hello.txt");

            let mut username_file = match username_file_result {
                Ok(file) => file,
                Err(e) => return Err(e),
            };

            let mut username = String::new();

            match username_file.read_to_string(&mut username) {
                Ok(_) => Ok(username),
                Err(e) => Err(e),
            }
        }
        }
        ```

65. Does the following code produce an error? Why?

        ```rust
        use std::fs::File;

        fn main() {
            let greeting_file = File::open("hello.txt")?;
        }
        ```

66. What can be the return type of a main function?
67. Recall the guessing game from chapter 2. Rewrite it to check the the user inputs number from within [1, 100] range. To do so, write a custom `Guess` type with constructor responsible to panic if the value is not in the expected range.
