# V4 - Programski jezik Rust

## Strukture, enumeracije i podudaranje obrazaca

1. Simplify the instantiation of the struct `User` inside the function `build_user`:
    ```rust
    struct User {
      active: bool,
      username: String,
      email: String,
      sign_in_count: u64,
    }

    fn build_user(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }
    ```

2. Assume that the `user1` is properly initialized. Simplify the following initialization. Can `user1` be used after the `user2` initialization? Why?

    ```rust
    let user2 = User {
          active: user1.active,
          username: user1.username,
          email: String::from("another@example.com"),
          sign_in_count: user1.sign_in_count,
    };
    ```

3. Do `black` and `origin` take the same type?

    ```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    fn main() {
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
    }
    ```

4. Is the following code valid?

    ```rust
    struct AlwaysEqual;

    fn main() {
        let subject = AlwaysEqual;
    }
    ```

5. Define a struct `Rectangle` with two fields `height` and `weight`. Afterwards, define the function/method responsible to calculate the area of a rectangle received as an argument. Enable debugging of `Rectangle` by using #[derive] attribute.
6. Implement the same solution above using methods.
7. On which types can a method be defined?
8. What is the difference between `self` and `Self`?
9. Can a method take the ownership of `self`?
10. When should a method take the ownership of `self`?
11. Implement an associated function of `Rectangle` type called `square` (not a method) responsible to create an instance of a `Rectangle` representing a square. The example of calling using this function might be: `let sq = Rectangle::square(3)`;
12. Can a struct have multiple `impl` blocks?
13. What are enumerations in Rust?
14. Write the following example in a more concise way by using enums only:

    ```rust
    fn main() {
      enum IpAddrKind {
          V4,
          V6,
      }

      struct IpAddr {
          kind: IpAddrKind,
          address: String,
      }

      let home = IpAddr {
          kind: IpAddrKind::V4,
          address: String::from("127.0.0.1"),
      };

      let loopback = IpAddr {
          kind: IpAddrKind::V6,
          address: String::from("::1"),
      };
    }
    ```

15. Does the following code produce an error? Why?

    ```rust
    fn main() {
      enum IpAddr {
          V4(u8, u8, u8, u8),
          V6(String),
      }

      let home = IpAddr::V4(127, 0, 0, 1);

      let loopback = IpAddr::V6(String::from("::1"));
    }
    ```

16. Does Rust have a concept of a `null` value?
17. Fix an error in the following snippet:
    ```rust
    fn main() {
      let _absent_number = None;
    }
    ```

18. What are the advantages of using `Option<T>` over null?
19. What are the `match` arms?
20. What is the return value of a `match` expression?
21. What does the following programs prints on the stdout?

    ```rust
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => 1,
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("State quarter from {:?}!", state);
                25
            }
        }
    }

    fn main() {
        value_in_cents(Coin::Quarter(UsState::Alaska));
    }
    ```

22. What is the output of the following program execution:

    ```rust
    fn main() {
      fn plus_one(x: Option<i32>) -> Option<i32> {
          match x {
              None => None,
              Some(i) => Some(i + 1),
          }
      }

      let five = Some(5);
      let six = plus_one(five);
      let none = plus_one(None);
      
      println!("five: {:#?}, six: {:#?}, none: {:#?}", five, six, none)
    }
    ```

23. What is the output the this program execution? Why?

    ```rust
    fn main() {
      fn plus_one(x: Option<i32>) -> Option<i32> {
          match x {
              Some(i) => Some(i + 1),
          }
      }

      let five = Some(5);
      let six = plus_one(five);
      let none = plus_one(None);
      
      println!("five: {:#?}, six: {:#?}, none: {:#?}", five, six, none)
    }
    ```

24. Fix the error the code below produces:

    ```rust
    fn main() {
      let dice_roll = 9;
      match dice_roll {
          3 => add_fancy_hat(),
          7 => remove_fancy_hat(),
      }

      fn add_fancy_hat() {}
      fn remove_fancy_hat() {}
    }
    ```

25. Write the following example more concisely using if let construct:

    ```rust
    fn main() {
      let config_max = Some(3u8);
      match config_max {
          Some(max) => println!("The maximum is configured to be {}", max),
          _ => (),
      }
    }
    ```

26. What are the tradeoffs of using `if let` compared to `match`?
27. Rewrite the following code to use `if let` construct:

    ```rust
    #[derive(Debug)]
    enum UsState {
        Alabama,
        Alaska,
        // --snip--
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn main() {
        let coin = Coin::Penny;
        let mut count = 0;
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }
        println!("The count is: {}", count)
    }
    ```
