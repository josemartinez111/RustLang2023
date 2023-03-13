// ! tcpserver/src/main.rs
use std::net::{TcpListener};
//? ********************************************************

fn handle_connection(connection_listener: TcpListener) {
	// Iterate over incoming connections using a closure
	connection_listener.incoming().for_each(|stream| {
		// Use a match to handle possible errors with the stream
		match stream {
			Ok(stream) => {
				// If the stream is valid, handle it here
				println!(
					"Client connected: {}",
					// Use unwrap_or_else to handle possible errors with peer_addr()
					stream.peer_addr().unwrap_or_else(|err|
						panic!("Failed to get peer address: {}", err)
					)
				)
			}
			Err(err) => {
				// If there is an error, print a message
				eprintln!("failed: {}", err);
			}
		}
	})
}
//? ********************************************************

fn main() {
	// In Rust, TcpListener is a type that represents a TCP socket server that listens
	// for incoming connections. It is part of the standard library in Rust and is used
	// for creating server applications that listen for connections on a specific port and
	// IP address.TCP stands for Transmission Control Protocol, which is one of the core
	// protocols of the Internet Protocol (IP) suite. TCP is responsible for establishing
	// and maintaining connections between network applications, ensuring reliable transmission
	// of data between them, and managing flow control and congestion control to prevent
	// network congestion and loss of data.In web development, TCP is used extensively for
	// communication between servers and clients over the internet. When a client makes a
	// request to a server, it sends the request over a TCP connection. The server receives
	// the request, processes it, and sends a response back to the client over the same TCP
	// connection. This is the basic mechanism behind HTTP, the protocol used for most web
	// traffic. TCP is also used for other web protocols such as FTP, SSH, and Telnet.
	let connection_listener = TcpListener::bind("127.0.0.1:3000")
		.unwrap_or_else(|err| {
			panic!("Failed to bind to socket: {}", err)
		});
	
	println!("Running on port https://localhost:3000");
	handle_connection(connection_listener);
}
//? ********************************************************
