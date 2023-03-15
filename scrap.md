```rust
// !I like more explicit 'lifetime naming conventions
fn error_type_str<'error>(print_err: &str, error: &'error &str) -> &'error str {
	eprintln!("ERROR: {}", print_err);
	// !Return the `error` string slice directly. with a lifetime
	// !When passing the `arg` make sure to use &arg
	error
}
```