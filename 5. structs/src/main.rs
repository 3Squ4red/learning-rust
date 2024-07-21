#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

// methods are similar to functions except that they're tied to an instance of a struct
// implementation blocks will house the functions and methods associated with our struct
impl Rectangle {
    fn area(&self) -> i32 {
        return self.width * self.height;
    }

    fn can_hold(&self, another: &Rectangle) -> bool {
        return self.area() >= another.area();
    }
}

// structs allow us to have multiple impl blocks
impl Rectangle {
    // inside our implementation block we can also define associated functions unlike methods associative functions are not tied to an instance of our struct
    // associated functions do not need to have the `&self` param at first
    fn square(size: i32) -> Rectangle {
        return Rectangle {
            width: size,
            height: size,
        };
    }
}

fn main() {
    // To modify even a single field of a struct var, you'd have to make that entire var mutable.
    let mut user1 = User {
        email: String::from("wv5uZ@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("helloworld23");

    let user2 = build_user(
        String::from("wv5uZ@example.com"),
        String::from("someusername123"),
    );

    // structs can be destructured
    let user3 = User {
        // email: String::from("wv5uZ@example.com"),
        // username: String::from("someusername123"),
        ..user2
    };

    println!("{}", user3.username); //  would be same as user2

    // --------------- Tuple Structs ---------------
    // we can also create Structs without the name field. These are called tuple structs
    // tuple structs are useful when you want your entire tuple to have a name and be of different type than other tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let rect = Rectangle {
        height: 30,
        width: 40,
    };

    let rect2 = Rectangle {
        height: 30,
        width: 20,
    };

    let rect3 = Rectangle {
        height: 60,
        width: 20,
    };

    let rect4 = Rectangle {
        height: 60,
        width: 30,
    };

    let square = Rectangle::square(5);

    println!("Square rect is {:?}", square);

    println!("can {:?} go into rect: {}", rect2, rect.can_hold(&rect2));
    println!("can {:?} go into rect: {}", rect3, rect.can_hold(&rect3));
    println!("can {:?} go into rect: {}", rect4, rect.can_hold(&rect4));

    println!("The area of rect is {}", areaT((30, 50)));
    println!("The area of rect is {}", rect.area());

    println!("{}", rect.height); // if this works it'd mean `areaS` didn't take ownership of rect

    // Display traits specify how something should be printed

    println!("{:?}", user3);
    println!("{:#?}", rect);

    let arr = [2; 10];
    println!("{:?}", arr);
}

fn build_user(email: String, username: String) -> User {
    User {
        email, // Field-init-shorthand syntax. Works because the value and the field have the same name
        username, // Field-init-shorthand syntax
        sign_in_count: 1,
        active: true,
    }
}

fn areaT(dimensions: (i32, i32)) -> i32 {
    return dimensions.0 * dimensions.1;
}
