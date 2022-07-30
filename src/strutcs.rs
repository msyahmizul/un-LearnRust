// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }
// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        return self.height * self.height;
    }
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    // Associated Functions
    // can use to run without init the structs
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn main() {
    // let mut user = User {
    //     email: String::from("Syahmi Zulkifli"),
    //     username: String::from("syahmizul"),
    //     active: true,
    //     sign_in_count: 1,
    // };

    // // fill the empty struct with user var
    // let user2 = User {
    //     email: String::from("another@example.com"),
    //     username: String::from("anotherusername567"),
    //     ..user
    // };
    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     rect1.area()
    // );
    // let rect1 = Rectangle {
    //     width: 30,
    //     height: 50,
    // };
    // let rect2 = Rectangle {
    //     width: 10,
    //     height: 40,
    // };
    // let rect3 = Rectangle {
    //     width: 60,
    //     height: 45,
    // };

    // println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    // println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
