// ENUM
enum IpAddrKind {
    V4,
    V6,
}

// STRUCT WITH ENUM INSIDE
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// ENUM WITH TYPE ASSOCIATED
enum IpAddrV2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn route(ip_kind: IpAddrKind) {}

// STRUCTS DEFINED INSIDE ENUMS
struct Ipv4Addr {
    // --snip--
}

struct Ipv6Addr {
    // --snip--
}

enum IpAddr2 {
    V4(Ipv4Addr),
    V6(Ipv6Addr),
}

fn main() {
    println!("Hello, Enums!");

    // INSTANCE
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // FUNCTION
    // Can take either values of the enum type
    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // STRUCT WITH ENUM INSIDE
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // ENUM WITH TYPE ASSOCIATED
    let home = IpAddrV2::V4(127, 0, 0, 1);
    let loopback = IpAddrV2::V6(String::from("::1"));
}
