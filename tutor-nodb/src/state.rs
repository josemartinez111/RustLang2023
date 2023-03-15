// !src/state.rs
use std::sync::Mutex;
//? ********************************************************
// !The mutexes in this module implement a strategy called "poisoning"
// !where a mutex is considered poisoned whenever a thread panics while
// !holding the mutex. Once a mutex is poisoned, all other threads are
// !unable to access the data by default as it is likely tainted
// !(some invariant is not being upheld).For a mutex, this means that
// !the lock and try_lock methods return a Result which indicates whether
// !a mutex has been poisoned or not. Most usage of a mutex will simply
// !unwrap() these results, propagating panics among threads to ensure that
// !a possibly invalid invariant is not witnessed. A poisoned mutex, however,
// !does not prevent all access to the underlying data. The PoisonError type
// !has an into_inner method which will return the guard that would have
// !otherwise been returned on a successful lock. This allows access to the data,
// !despite the lock being poisoned.
pub struct AppSate{
	pub health_check_response: String,
	pub visit_count: Mutex<u32>
}
//? ********************************************************
//? ********************************************************