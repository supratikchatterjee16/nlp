#[warn(off)]
pub mod nlp;

pub use nlp::{*};
use std::collections::LinkedList;

struct NLPInstance{
	dictionary : LinkedList<Token>,
	input : String,
	input_type : u8,
	output : String,
	output_type : u8,//0 for file, 1 for net for both input & output
	
}
impl Instance{
	//private common functions
	//public common functions
	//Constructors
	pub fn new() -> NLPInstance{
		NLPInstance{
			dictionary : LinkedList::new(),
			input : String::new(), 
			input_type : 0,
			output : String::new(), 
			output_type : 0
		}//End of constructing NLPInstance
	}
	//Setters
	fn set_dictionary(filepath : &String){}
	fn set_input(src : &String){}
	fn set_output(dest : &String){}
	fn set_pipe(){}
	//Getters
	fn get_input(){}
	fn get_output(){}
	fn get_pipe(){}
	fn get_status(){}
}