#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6
}

// how to represent an actual address?
// perhaps a struct with a kind and address.
struct IpAddrNaive {
    kind: IpAddrKind,
    addr: String
}

// better option (even supports different structure of each type)
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String)
}
// e.g. can define 
// fn route(ip: IpAddr) {}

/* Another option
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
} 
*/


fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    println!("{:?} {:?}", four, six);

    let home_naive = IpAddrNaive {
        kind: IpAddrKind::V4,
        addr: String::from("127.0.0.1")
    };
    println!("naive: {:?} {:?}", home_naive.kind, home_naive.addr);

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    println!("{:?} {:?}", home, loopback);


    // standard library Option (same as std::optional<T> in C++)
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }
    let some_number = Some(5); // type inferred
    let some_char = Some('e'); // type inferred
    let absent_number: Option<i32> = None; // need to specify type since it can't be inferred
    println!("{:?} {:?} {:?}", some_number, some_char, absent_number);

    // let sum = some_number + 5; // won't work, need to unbox
}


/* Another enum:

enum Message {
    Quit, // no associated data
    Move { x: i32, y: i32 }, // named associated data
    Write(String), // single string
    ChangeColor(i32, i32, i32), // 3 i32s
}

equivalent:

struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/