use std::io::prelude::*;
use std::io::Read;
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;
    // ["ping", "hello"] -> *2\r\n$4\r\nPING\r\n$4\r\nALGO\r\n
    let redis_simple_ping =
        b"*2\r\n$4\r\nPING\r\n$4\r\nHELLO\r\n";
    stream.write(redis_simple_ping)?;
    let mut buffer = [0; 512];
    stream.read(&mut buffer[..])?;
    let string_msg = str::from_utf8(&buffer).unwrap();
    println!("msg: {:?}", string_msg);
    Ok(())
}
