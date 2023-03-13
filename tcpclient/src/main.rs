// ! tcpclient/src/main.rs
use std::net::TcpStream;
//? ********************************************************
//? ********************************************************


fn main() {
	let _stream = TcpStream::connect("localhost:3000")
		.unwrap_or_else(|err| {
			panic!("Could not connect to host: {}", err);
		});
}
//? ********************************************************

