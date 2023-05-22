# V6 - Programski jezik Rust

## Generički tipovi, osobine, životni vek

1. Instead of two functions `largest_i32` and `largest_char`, write the single function using generics that provides the same
  functionality for the following `main` method.

    ```rust
        fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);
        assert_eq!(*result, 100);

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest_char(&char_list);
        println!("The largest char is {}", result);
        assert_eq!(*result, 'y');
    }
    ```

2. Find and fix the error in the following snippet.
    ```rust
    struct Point<T> {
        x: T,
        y: T,
    }

    fn main() {
        let wont_work = Point { x: 5, y: 4.0 };
    }
    ```

3. Is it allowed to use generics over enums? If so, provide an example.
4. What does the following code display on the *stdout*?

    ```rust
    struct Point<X1, Y1> {
        x: X1,
        y: Y1,
    }

    impl<X1, Y1> Point<X1, Y1> {
        fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    fn main() {
        let p1 = Point { x: 5, y: 10.4 };
        let p2 = Point { x: "Hello", y: 'c' };

        let p3 = p1.mixup(p2);

        println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    }
    ```

5. Is there a runtime overhead or cost to using generics? Why?
6. What concept is similar to interface in other languages like (e.g. Java)? What is the difference?
7. Is it possible to implement `Display` trait on `Vec<T>`? Why?
8. Is it possible to call the default implementation from an overriding implementation of the same method?
9. What does the following code snippet print to the stdout?

    ```rust
    pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }


    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }


    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }

        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }


    fn main() {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());
        
        
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
        }
     ```

10. Simplify the following snippet by using the trait bound syntax.

    ```rust
    pub fn notify(item1: &impl Summary, item2: &impl Summary)
    ```

11. Write the equivalent snippet by using trait bound syntax.
    ```rust
    pub fn notify(item: &(impl Summary + Display)) {
    ```

12. Write the equivalent code snippet by using the `where` clause.

    ```rust
    fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    ```

13. Does the following code produce an error?

    ```rust
        pub trait Summary {
        fn summarize_author(&self) -> String;

        fn summarize(&self) -> String {
            format!("(Read more from {}...)", self.summarize_author())
        }
    }


    pub struct Tweet {
        pub username: String,
        pub content: String,
        pub reply: bool,
        pub retweet: bool,
    }

    impl Summary for Tweet {
        fn summarize_author(&self) -> String {
            format!("@{}", self.username)
        }
    }


    pub struct NewsArticle {
        pub headline: String,
        pub location: String,
        pub author: String,
        pub content: String,
    }

    impl Summary for NewsArticle {
        fn summarize_author(&self) -> String {
            format!("@{}", self.author)
        }

        fn summarize(&self) -> String {
            format!("{}, by {} ({})", self.headline, self.author, self.location)
        }
    }


    fn main() {
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());
        
        
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }
    
    
    fn returns_summarizable(switch: bool) -> impl Summary {
        if switch {
            NewsArticle {
                headline: String::from(
                    "Penguins win the Stanley Cup Championship!",
                ),
                location: String::from("Pittsburgh, PA, USA"),
                author: String::from("Iceburgh"),
                content: String::from(
                    "The Pittsburgh Penguins once again are the best \
                    hockey team in the NHL.",
                ),
            }
        } else {
            Tweet {
                username: String::from("horse_ebooks"),
                content: String::from(
                    "of course, as you probably already know, people",
                ),
                reply: false,
                retweet: false,
            }
        }
    }
    ```

14. What does `blanket implementation` mean?
15. What is "reference lifetime"?
16. Does the following code snippet produce an error? If so, fix it.

    ```rust
    fn main() {
        let r: i32;
        println!("r: {}", r);
    }
    ```

17. Does the following code produce an error? Why? Hint: *dangling reference*.

    ```rust
    fn main() {
        let r;

        {
            let x = 5;
            r = &x;
        }

        println!("r: {}", r);
    }
    ```

18. How does the Rusts `borrow checker` work?
19. Find and fix the errors in the following code snippet.

    ```rust
    fn main() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    fn longest(x: &str, y: &str) -> &str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```

20. Is the following code valid? Why?

    ```rust
    fn main() {
        let string1 = String::from("long string is long");

        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```

21. Does the following code compile successfully? Why?

    ```rust
    fn main() {
        let string1 = String::from("long string is long");
        let result;
        {
            let string2 = String::from("xyz");
            result = longest(string1.as_str(), string2.as_str());
        }
        println!("The longest string is {}", result);
    }

    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    ```

22. Fix the error in the following code snippet.

    ```rust
    fn main() {
        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }

    fn longest<'a>(x: &str, y: &str) -> &'a str {
        let result = String::from("really long string");
        result.as_str()
    }
    ```

23. Fix the error in the following code snippet.

    ```rust
    struct ImportantExcerpt {
        part: &str,
    }

    fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    ```

24. What are input and output lifetimes?
25. What are the *lifetime elision* rules?
26. Do we need explicit lifetime annotations in the following example? Why?

    ```rust
    fn first_word(s: &str) -> &str {
    ```

27. Fix errors in the following code snippet.

    ```rust
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    impl ImportantExcerpt {
        fn level(&self) -> i32 {
            3
        }
    }

    impl ImportantExcerpt {
        fn announce_and_return_part(&self, announcement: &str) -> &str {
            println!("Attention please: {}", announcement);
            self.part
        }
    }

    fn main() {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
    }
    ```

28. Write an equivalent code snippet that explicitly uses lifetimes.

    ```rust
    let s: &str = "I have a static lifetime.";
    ```

29. How long does the string literal live for?
30. What is the difference between using associated types and generics when implementing traits?
31. Does the following code panic?

    ```rust
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Point {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    fn main() {
        assert_eq!(
            Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
            Point { x: 3, y: 3 }
        );
    }
    ```

32. Why the following example uses `Add<Meters>` and not only `Add` as the previous example?

    ```rust
    use std::ops::Add;

    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }
    ```

33. What is the output of the following program?

    ```rust
    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    fn main() {
        let person = Human;
        person.fly();
    }
    ```

34. Rewrite the `main` function from the previous example to call all try `fly` methods.
35. What is the output of the following program?

    ```rust
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    fn main() {
        println!("A baby dog is called a {}", Dog::baby_name());
    }
    ```

36. Rewrite the `main` function of the previous example to call both `baby_name` methods.
37. The following code does not compile. Implement the necessary trait on the `Point` type.
    ```rust
    use std::fmt;

    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    struct Point {
        x: i32,
        y: i32,
    }

    impl OutlinePrint for Point {}
    ```
38. Implement the `Display` trait on the `Vector<T>` type.
39. What does the following program show on the stdout?

    ```rust
    fn main() {
        type Kilometers = i32;
        let x: i32 = 5;
        let y: Kilometers = 5;
        println!("x + y = {}", x + y);
    }
    ```

40. What is the *never type*?
41. what are diverging functions?
42. What is the type of the `guess` variable in the following snippet?

    ```rust
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    ```

43. What is the type of the following `loop` in the snippet below?

    ```rust
        print!("forever ");

        loop {
            print!("and ever ");
        }
    ```

44. Does the following code compile? Why?

    ```rust
    fn main() {
        let s1: str = "Hello there!";
        let s2: str = "How's it going?";
    }
    ```

45. Are traits dynamically sized types?
46. What trait can be helpful when dealing with dynamically sized types (DST)?
47. Can generic function work with dynamically sized types by default?
