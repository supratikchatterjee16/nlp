pub mod tag;
pub mod token;
pub mod sentence;
pub mod paragraph;
pub mod context;

#[cfg(test)]
use self::context::Context;
#[test]
fn sammple_context_creation() {
	let context = Context::from_str("Crap begets crap".to_string());
	assert_eq!(context.get_count(), 1);
}