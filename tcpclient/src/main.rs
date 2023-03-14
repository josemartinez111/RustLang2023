use std::borrow::Cow;
use std::io;
use std::io::{Read, Write};
// ! tcpclient/src/main.rs
use std::net::TcpStream;
//? ********************************************************

type Void = ();
//? ********************************************************

fn main() -> io::Result<Void> {
	// *Connect to the server
	let mut client_stream = TcpStream::connect("localhost:3000")?;
	// *Write data to the server
	let msg = "Hola".as_bytes();
	client_stream.write(msg)?;
	
	// *Read the server's response
	let mut incoming_data_buffer = [0; 5];
	let num_of_bytes_read = client_stream.read(&mut incoming_data_buffer)?;
	
	// *Print & format the response from the server
	// ?A clone-on-write smart pointer.
	// ?The type Cow is a smart pointer providing clone-on-write functionality:
	// it can enclose and provide immutable access to borrowed data, and clone
	// the data lazily when mutation or ownership is required. The type is designed
	// to work with general borrowed data via the Borrow trait. Cow implements Deref,
	// which means that you can call non-mutating methods directly on the data it encloses.
	// If mutation is desired, to_mut will obtain a mutable reference to an owned value,
	// cloning if necessary. If you need reference-counting pointers, note that
	// Rc::make_mut and Arc::make_mut can provide clone-on-write functionality as well.
	let response: Cow<str> = String::from_utf8_lossy(&incoming_data_buffer[..num_of_bytes_read]);
	println!("Got response from server: {}", response);
	
	Ok(())
}
//? ********************************************************

