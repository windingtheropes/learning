enum IpAddrKind { // there are only two possible ip address types, and an item can only be one at a time.
    V4(u8, u8, u8, u8), // since ipv4 address can be up to 255.255.255.255
    V6(String),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind::V4(String::from(127, 0, 0, 1));
    let loopback = IpAddrKind::V6(String::from("::1"));
}

fn route(ip_kind: IpAddrKind) {}
