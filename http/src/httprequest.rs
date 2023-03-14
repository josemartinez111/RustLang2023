// !http/src/httprequest.rs
use std::collections::HashMap;
use std::str::{Split, SplitWhitespace};
//? ********************************************************

#[derive(Debug, PartialEq)]
pub enum Resource {
	Path(String)
}

#[derive(Debug)]
pub struct HttpRequest {
	pub method: Method,
	pub version: Version,
	pub resource: Resource,
	pub headers: HashMap<String, String>,
	pub msg_body: String,
}

enum LineType {
	Request,
	Header,
	Blank,
	Body,
}

#[derive(Debug, PartialEq)]
pub enum Method {
	Get,
	Post,
	Uninitialized,
}

#[derive(Debug, PartialEq)]
pub enum Version {
	V1_1,
	V2_0,
	Uninitialized,
}
//? ********************************************************

// Implementing the from method in From trait enables us to
// read the method string from the HTTP request line, and
// convert it into Method::Get or Method::Post variant.
impl From<&str> for Method {
	fn from(value: &str) -> Self {
		match value {
			"GET" => Method::Get,
			"POST" => Method::Post,
			_ => Method::Uninitialized,
		}
	}
}

impl From<&str> for Version {
	fn from(value: &str) -> Self {
		match value {
			"HTTP/1.1" => Version::V1_1,
			_ => Version::Uninitialized
		}
	}
}

impl From<String> for HttpRequest {
	fn from(req: String) -> Self {
		let mut parsed_method = Method::Uninitialized;
		let mut parsed_version = Version::V1_1;
		let mut parsed_resource = Resource::Path("".to_string());
		let mut parsed_headers = HashMap::new();
		let mut parsed_msg_body: &str = "";
		
		// ?Read each line in the incoming HTTP request
		// !lines(): An iterator over the lines of a string, as string slices.
		req.lines().for_each(|line| {
			// ?Determine the type of line
			let line_type = match () {
				// ?If the line contains "HTTP", it is the request line
				_ if line.contains("HTTP") => LineType::Request,
				// ?If the line contains ":", it is a header line
				_ if line.contains(":") => LineType::Header,
				// ?If the line is blank, it is a blank line
				_ if line.trim().is_empty() => LineType::Blank,
				// ?Otherwise, it is part of the message body
				_ => LineType::Body
			};
			
			// !Take the appropriate action based on the type of line
			match line_type {
				// ?If the line is the request line, parse it and update the parsed fields
				LineType::Request => {
					let (method, resource, version): (Method, Resource, Version) = process_req_line(line);
					parsed_method = method;
					parsed_resource = resource;
					parsed_version = version;
				}
				// If the line is a header line, parse it and add it to the parsed headers
				LineType::Header => {
					let (key, value) = process_header_line(line);
					parsed_headers.insert(key, value);
				}
				// If the line is blank, do nothing
				LineType::Blank => {}
				// Otherwise, treat the line as part of the message body
				LineType::Body => {
					parsed_msg_body = line;
				}
			}
		});
		
		// !Parse the incoming HTTP request into HttpRequest struct
		Self {
			method: parsed_method,
			version: parsed_version,
			resource: parsed_resource,
			headers: parsed_headers,
			// !The `to_string` method creates a new String instance
			// !that contains the same content as the string slice `&str`.
			msg_body: parsed_msg_body.to_string(),
		}
	}
}

//? ********************************************************

fn process_req_line(req_line: &str) -> (Method, Resource, Version) {
	// ?Parse the request line into individual chunks split by whitespaces.
	let mut words: SplitWhitespace = req_line.split_whitespace();
	
	// ?Extract the HTTP method from the first part of the request line
	let method: &str = words.next().unwrap_or_else(|| {
		error_type_str(
			"Error: Missing HTTP method",
			&"MISSING HTTP METHOD",
		)
	});
	
	// ?Extract the resource (URI/URL) from second part of the request line
	let resource: &str = words.next().unwrap_or_else(|| {
		error_type_str(
			"Error: Missing resource",
			&"NO RESOURCE FOUND",
		)
	});
	
	// ?Extract the HTTP version from third part of the request line
	let version: &str = words.next().unwrap_or_else(|| {
		error_type_str(
			"Error: Missing HTTP version",
			&"MISSING HTTP VERSION",
		)
	});
	
	let tuple_result = (
		method.into(),
		Resource::Path(resource.to_string()),
		version.into(),
	);
	
	tuple_result
}

// !I like more explicit 'lifetime naming conventions
fn error_type_str<'error>(print_err: &str, error: &'error &str) -> &'error str {
	eprintln!("ERROR: {}", print_err);
	// !Return the `error` string slice directly. with a lifetime
	// !When passing the `arg` make sure to use &arg
	error
}

fn process_header_line(item: &str) -> (String, String) {
	// ?Parse the header line into words split by separator (':')
	let mut header_items: Split<&str> = item.split(":");
	
	// ?Creating a key/value pair
	let mut key: String = String::from("");
	let mut value: String = String::from("");
	
	// ?Extract the key part of the header
	if let Some(key_ok) = header_items.next() {
		key = key_ok.to_string();
	}
	// ?Extract the value part of the header
	if let Some(value_ok) = header_items.next() {
		value = value_ok.to_string();
	}
	
	(key, value)
}
//? ********************************************************

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_read_http() {
		let request: &str = "GET /greeting HTTP/1.1\r\nHost:localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept:*/*\r\n\r\n";
		let s: String = String::from(request);
		let mut headers_expected = HashMap::new();
		headers_expected.insert("Host".into(), " localhost".into());
		headers_expected.insert("Accept".into(), " */*".into());
		headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());
		let req: HttpRequest = s.into();
		assert_eq!(Method::Get, req.method);
		assert_eq!(Version::V1_1, req.version);
		assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
		assert_eq!(headers_expected, req.headers);
	}
	
	#[test]
	fn test_method_into() {
		let method: Method = "GET".into();
		assert_eq!(method, Method::Get);
	}
}

#[test]
fn test_version_into() {
	// In this specific case, calling into() on the string literal "HTTP/1.1"
	// is equivalent to calling String::from("HTTP/1.1"), which converts the
	// string literal into a String object.However, since the target type for
	// the conversion is Version, not String, the into() method will actually
	// call the From trait's from() method, which can convert a value of one
	// type into another. In this case, the from() method of the Version type
	// will be called to convert the String into a Version object.The into()
	// method is commonly used in Rust code to simplify conversions between types,
	// especially in cases where the conversion can be inferred by the Rust
	// compiler. By defining the From and Into traits for your types, you can
	// enable seamless and intuitive type conversions in your code, which can
	// help reduce boilerplate and make your code more expressive.
	let version: Version = "HTTP/1.1".into();
	assert_eq!(version, Version::V1_1)
}

#[test]
fn test_method_into() {
	let method: Method = "GET".into();
	assert_eq!(method, Method::Get);
}
//? ********************************************************











