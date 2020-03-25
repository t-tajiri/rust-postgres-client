use std::net::TcpStream;
use std::io::Result;
use std::io::Error;

pub fn main() -> Result<TcpStream> {
    let stream = TcpStream::connect("*");
    stream
}

#[test]
fn should_passed_if_invalid_address () {
    assert_eq!(true, main().is_err());
}

