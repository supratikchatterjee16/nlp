mod tag;
mod token;
mod sentence;
mod paragraph;
mod context;

pub use crate::nlp::structure::tag::Tag;
pub use crate::nlp::structure::token::Token;
pub use crate::nlp::structure::sentence::Sentence;
pub use crate::nlp::structure::paragraph::Paragraph;
pub use crate::nlp::structure::context::Context;

#[cfg(test)]
use self::context::Context;
#[test]
fn sammple_context_creation() {
	let context = Context::from_str("Crap begets crap".to_string());
	assert_eq!(context.get_count(), 2);
}