fn main() {
    // Congratulations, you finished the first exercise 🎉
    // As an introduction to Rustlings, the first exercise only required
    // entering `n` in the terminal to go to the next exercise.
    let user = User {
        active: true,
        email: String::from("some email"),
        username: String::from("paprik"),
        sign_in_count: 1,
    };
    let user2 = User {
        email: String::from("new email"),
        ..user
    };
    user.email;
    user.username;
    user.active;

    struct Point(String, i32, i32);
    let point = Point(String::from("hello"), 0, 0);
    // let Point(x, y, z) = point;
    point.0;

    let num: u32 = 1;

    struct Rectangle {
        width: u32,
        heigth: u32,
    }
    impl Rectangle {
        fn area(self) -> u32 {
            self.width * self.heigth
        }
    }
    let rectangle = Rectangle {
        width: 30,
        heigth: 50,
    };
    rectangle.area();
    rectangle.width;

    let num = 0i32;

    let wallet = Some(60);
    let Some(money) = wallet else {
        return;
    };
}

struct User {
    active: bool,
    email: String,
    username: String,
    sign_in_count: u64,
}
trait Greeting {
    fn greet(&self);
}
impl Greeting for User {
    fn greet(&self) {
        println!("Hello");
    }
}

fn hello<T: Greeting>(user: T) -> T {
    user.greet();
    user
}

// use std::error;
// use std::fs::File;
// use std::io;
// fn test() -> Result<(), error::Error> {
//     File::open("")?;
//     Ok(())
// }
