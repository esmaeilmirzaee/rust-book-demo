enum IpAddrKindV1 {
  V4,
  V6,
}

enum IpAddrKindV2 {
  V4(u8, u8, u8, u8),
  V6(String),
}

struct IpAddr {
  kind: IpAddrKindV1,
  address: String,
}

impl IpAddrKindV2 {
  fn collision(&self, other: &IpAddrKindV1) -> bool {
    false
  }

  fn ping(&self) {
    // println!("Pinging {}.{}.{}.{}...", self::v4.0, 1, 2, 3);
  }
}

// Defining message via enumeration is so elegant
enum Message {
  Quit,
  Write(String),
  Move {x: i32, y: i32},
  ChangeColor(u32, u32, u32),
}

// It is possible to use struct to define different types but it's untidy.
struct QuitMessage;
struct WriteMessage(String);
struct MoveMessage {
  x: i32,
  y: i32,
}
struct ChangeColorMessage(u32, u32, u32);
// -----------------------------------------

fn route(ip_kind: IpAddrKindV1) {
  println!("{}", ip_kind::V4);
}

fn main() {
  let home = IpAddrKindV2::V4(192, 168, 2, 3);
  let home_addr = IpAddr {
    kind: IpAddrKindV1::V4,
    address: String::from("192.168.2.3"),
  };
  home.ping();
}
