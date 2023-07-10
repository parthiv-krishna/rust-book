struct User {
    active: bool,
    // for now, will have structs own all objects
    // (String instead of &str). to hold references,
    // will need lifetimes (chapter 10)
    username: String,
    email: String,
    sign_in_count: u64,
}

// tuples
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit-like struct: behaves like empty tuple ()
struct AlwaysEqual;

fn main() {
    let user1 = User {
        // order doesn't matter
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    print_user(&user1);

    let mut user2 = User {
        username: String::from("anotherusername123"),
        ..user1 // copy all other fields from user1
    };
    print_user(&user2);
    user2.sign_in_count += 1; // mutate
    print_user(&user2);


    // tuples
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;

}

fn print_user(u: &User) {
    // read struct fields as expected
    println!("{} <{}> active:{}, sign_in_count:{}", u.username, u.email, u.active, u.sign_in_count);
}

