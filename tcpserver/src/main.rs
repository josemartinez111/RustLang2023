use std::io;
use std::io::{Read, Write};
// ! tcpserver/src/main.rs
use std::net::{TcpListener, TcpStream};
//? ********************************************************

type Void = ();
//? ********************************************************

fn handle_connection(mut stream: TcpStream) -> io::Result<Void> {
// *print a message indicating that a connection has been established
	println!("Client connected: {}", stream.peer_addr()?);
	
	// Read data from the client
	let mut incoming_data_buffer: [u8; 1024] = [0; 1024];
	let num_bytes_read: usize = stream.read(&mut incoming_data_buffer)?;
	
	// Write the same data back to the client
	stream.write(&incoming_data_buffer[0..num_bytes_read])?;
	Ok(())
}
//? ********************************************************

fn main() -> io::Result<Void> {
	// *bind the server to the address and port
	let connection_listener = TcpListener::bind("127.0.0.1:3000")?;
	// print a message indicating that the server is running if the bind was `Ok()`
	println!("Server is running on port: {}", connection_listener.local_addr()?);
	
	// handle incoming connections
	connection_listener.incoming().for_each(|stream| {
		match stream {
			Ok(ok_stream) => {
				// *handle the stream error here
				if let Err(err) = handle_connection(ok_stream) {
					eprintln!("Failed to handle connection: {}", err);
				}
			}
			Err(err) => eprintln!("Failed to accept client connection: {}", err),
		}
	});
	
	Ok(())
}
//? ********************************************************














