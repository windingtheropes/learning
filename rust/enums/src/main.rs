enum IpAddrKind { // there are only two possible ip address types, and an item can only be one at a time.
    V4(u8, u8, u8, u8), // since ipv4 address can be up to 255.255.255.255
    V6(String),
}

enum Message { // an item can only be one of these enums at a time. 
    Quit,
    Move {x: i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

// we can implement methods in enums as well

impl Message {
    fn call(&self) {
        // method body would go here
    }
}

// what the official ip address enums look like; you can add any kind of data into a variant, even structs.
struct Ipv4Addr {

}

struct Ipv6Addr {
    
}

enum IpAddr {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(String::from(127, 0, 0, 1));
    let loopback = IpAddrKind::V6(String::from("::1"));

    let m = Message::Write(String::from("hello world"));
    m.call()
}

fn route(ip_kind: IpAddrKind) {}
