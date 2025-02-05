#[derive(Debug)]
struct Ipv4Addr(u8,u8,u8,u8);
#[derive(Debug)]
struct Ipv6Addr(String);
#[derive(Debug)]
enum IpAddr {
    V4(u8,u8,u8,u8),
    V6(String),
    V4_copy(Ipv4Addr),
    V6_copy(Ipv6Addr),
    V8, //Just to try
}

fn main() {
   let home = IpAddr::V4(127,0,0,1);
   let loopback = IpAddr::V6(String::from("::1"));
   let home1 = IpAddr::V4_copy(Ipv4Addr(127,0,0,1));
   let loopback1 = IpAddr::V6_copy(Ipv6Addr(String::from("::1")));
   let imaginary_ip = IpAddr::V8;

   println!("{:?}", home);
   println!("{:?}", loopback);
   println!("{:?}", imaginary_ip);
   println!("{:?}", home1);
   println!("{:?}", loopback1);
}
