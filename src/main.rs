use std::io::prelude::*;
use std::io::Read;
use std::net::TcpStream;
use std::str;
use redis_cilent;

enum RedisCommand {
    Command(Vec<&'static str>)
}

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:6379")?;
    // ["ping", "hello"] -> *2\r\n$4\r\nPING\r\n$4\r\nhello\r\n
    let ping_command = vec!["PING", "Hello"];
    let redis_simple_ping = redis_cilent::to_string(&ping_command).unwrap();
    let redis_string = "+Algo\r\n";
    let redis_integer = ":3000\r\n";
    let a: Vec<String> = redis_cilent::from_str(&redis_simple_ping).unwrap();
    let b: String = redis_cilent::from_str(&redis_string).unwrap();
    let c: i64 = redis_cilent::from_str(&redis_integer).unwrap();
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);
    stream.write(&redis_simple_ping.into_bytes())?;
    let mut buffer = [0; 512];
    stream.read(&mut buffer[..])?;
    let string_msg = str::from_utf8(&buffer).unwrap();
    let pong: String = redis_cilent::from_str(string_msg).unwrap();
    println!("pong: {:?}", pong);
    Ok(())
}
