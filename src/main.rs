use std::io::prelude::*;
use std::io::Read;
use std::net::TcpStream;
use std::str;
use redis_cilent::to_string;

enum RedisCommand {
    Command(Vec<&'static str>)
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;
    // ["ping", "hello"] -> *2\r\n$4\r\nPING\r\n$4\r\nALGO\r\n
    let ping_command = vec!["PING", "LIB"];
    let redis_simple_ping = to_string(&ping_command).unwrap();
    stream.write(&redis_simple_ping.into_bytes())?;
    let mut buffer = [0; 512];
    stream.read(&mut buffer[..])?;
    let string_msg = str::from_utf8(&buffer).unwrap();
    println!("msg: {:?}", string_msg);
    Ok(())
}
