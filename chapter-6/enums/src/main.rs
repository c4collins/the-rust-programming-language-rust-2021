#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6,
}

#[derive(Debug)]
struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

fn main() {
    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddress {
        kind: IpAddressKind::V6,
        address: String::from("::1"),
    };
    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback)
}
